import { useEffect, useState } from "react"
import { useNavigate } from "@tanstack/react-router"
import { useQuery, useQueryClient } from "@tanstack/react-query"
import { Check } from "lucide-react"
import { toast } from "sonner"
import ProjectTag from "./project-tag"
import type { Project, Task } from "@/types"
import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
} from "@/components/ui/command"
import { invoke_tauri_command } from "@/lib/utils"

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
        queryKey: ['projects', 'commandBar'],
        queryFn: async () => {
            return await invoke_tauri_command('load_projects_command', { showArchivedProjects: false })
        }
    })

    const tasksQuery = useQuery({
        queryKey: ['tasks', 'commandBar'],
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
            (task.description?.toLowerCase().includes(searchLower)) ||
            (task.project_id && projectsQuery.data?.find((p: Project) => p.id === task.project_id)?.title.toLowerCase().includes(searchLower))
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
                                                navigate({ to: '/projects/$projectId', params: { projectId: `${project.id}` } as any })
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
                                            navigate({ to: '/tasks/$taskId', params: { taskId: `${task.id}` } as any })
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
                                            {task.project_id && (
                                                <div className="text-sm text-muted-foreground">
                                                    {projectsQuery.data?.find((p: Project) => p.id === task.project_id)?.title}
                                                </div>
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