import { useState, useEffect } from "react"
import { useNavigate } from "@tanstack/react-router"
import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
} from "@/components/ui/command"
import { useQuery, useQueryClient } from "@tanstack/react-query"
import { invoke_tauri_command } from "@/lib/utils"
import ProjectTag from "./project-tag"
import { Task, Project } from "@/types"
import { Check } from "lucide-react"
import { toast } from "sonner"

export function CommandBar() {
    const [open, setOpen] = useState(false)
    const [search, setSearch] = useState("")
    const navigate = useNavigate()
    const queryClient = useQueryClient()

    useEffect(() => {
        const down = (e: KeyboardEvent) => {
            if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
                e.preventDefault()
                setOpen((open) => !open)
            }
        }
        document.addEventListener("keydown", down)
        return () => document.removeEventListener("keydown", down)
    }, [])

    const projectsQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            return await invoke_tauri_command('load_projects_command', { showArchivedProjects: false })
        }
    })

    const tasksQuery = useQuery({
        queryKey: ['tasks'],
        queryFn: async () => {
            return await invoke_tauri_command('load_tasks_command', { includeCompleted: false })
        }
    })

    const filteredProjects = projectsQuery.data?.filter((project: Project) =>
        project.title.toLowerCase().includes(search.toLowerCase())
    )

    const filteredTasks = tasksQuery.data?.filter((task: Task) => {
        const searchLower = search.toLowerCase()
        return (
            task.title.toLowerCase().includes(searchLower) ||
            (task.description && task.description.toLowerCase().includes(searchLower)) ||
            (task.project && task.project.title.toLowerCase().includes(searchLower))
        )
    })

    const handleCompleteTask = async (taskId: string) => {
        const task = tasksQuery.data?.find((t: Task) => t.id === taskId)
        if (!task) return

        await invoke_tauri_command('complete_task_command', { taskId })
        queryClient.invalidateQueries({ queryKey: ['tasks'] })
        toast.success(`Completed task: ${task.title}`)
        setOpen(false)
    }

    return (
        <CommandDialog open={open} onOpenChange={setOpen}>
            <CommandInput
                placeholder="Type to search projects and tasks..."
                value={search}
                onValueChange={setSearch}
            />
            <CommandList>
                {search.length > 0 ? (
                    <>
                        <CommandEmpty>No results found.</CommandEmpty>
                        {filteredProjects && filteredProjects.length > 0 && (
                            <>
                                <CommandGroup heading="Projects">
                                    {filteredProjects.map((project: Project) => (
                                        <CommandItem
                                            key={project.id}
                                            onSelect={() => {
                                                navigate({ to: '/projects/$projectId', params: { projectId: project.id } })
                                                setOpen(false)
                                            }}
                                        >
                                            <ProjectTag projectId={project.id} />
                                        </CommandItem>
                                    ))}
                                </CommandGroup>
                                <CommandSeparator />
                            </>
                        )}
                        {filteredTasks && filteredTasks.length > 0 && (
                            <CommandGroup heading="Tasks">
                                {filteredTasks.map((task: Task) => (
                                    <CommandItem
                                        key={task.id}
                                        onSelect={() => {
                                            navigate({ to: '/tasks/$taskId', params: { taskId: task.id } })
                                            setOpen(false)
                                        }}
                                    >
                                        <div className="flex flex-col w-full">
                                            <div className="flex items-center justify-between">
                                                <span>{task.title}</span>
                                                <button
                                                    className="p-1 hover:bg-accent rounded-sm"
                                                    onClick={(e) => {
                                                        e.stopPropagation()
                                                        handleCompleteTask(task.id)
                                                    }}
                                                >
                                                    <Check className="h-4 w-4" />
                                                </button>
                                            </div>
                                            {task.project && (
                                                <span className="text-xs text-muted-foreground">
                                                    {task.project.title}
                                                </span>
                                            )}
                                        </div>
                                    </CommandItem>
                                ))}
                            </CommandGroup>
                        )}
                    </>
                ) : (
                    <CommandEmpty>Type to search...</CommandEmpty>
                )}
            </CommandList>
        </CommandDialog>
    )
} 