import React from 'react';
import {
    useMutation,
    useQuery,
    useQueryClient,
} from '@tanstack/react-query'
import { ColumnDef } from "@tanstack/react-table"
import { DataTable } from '@/components/data-table';
import { Checkbox } from '@/components/ui/checkbox';
import { Task } from '@/types';
import EditTaskDialog from '@/components/tasks-table/EditTaskDialog';
import ProjectTag from '@/components/project-tag';
import { invoke_tauri_command } from '@/lib/utils';
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import { Button } from '../ui/button';
import {
    Command,
    CommandGroup,
    CommandItem,
} from "@/components/ui/command"
import { Ellipsis } from 'lucide-react';
import { Link } from '@tanstack/react-router'
import { format, startOfWeek, addWeeks } from "date-fns"
import { Calendar } from "@/components/ui/calendar"
import { toast } from "sonner"


const columns: ColumnDef<Task>[] = [
    {
        id: "complete",
        size: 10,
        cell: ({ row }) => {
            const queryClient = useQueryClient()

            const markCompleteMutation = useMutation({
                mutationFn: async function () {
                    let res = await invoke_tauri_command('complete_task_command', { taskId: row.original.id });
                    return res
                },
                onSuccess: () => {
                    // Invalidate and refetch
                    queryClient.invalidateQueries({ queryKey: ['tasks'] })

                },
            });

            return (
                <Checkbox
                    checked={row.original.completed_at_utc != null}
                    onCheckedChange={() => {
                        markCompleteMutation.mutateAsync()
                    }}
                />
            )
        }
    },
    {
        id: "title",
        accessorKey: "title",
        header: "Task",
        size: 300,
        cell: ({ row }) => {
            return <div className='flex-col'>
                {row.original.parent_task_id && <ParentTaskLabel parentTaskId={row.original.parent_task_id} />}
                <Link
                    to="/tasks/$taskId" params={{ taskId: row.original.id }}
                >
                    <p className='hover:underline'>
                        {row.original.title}
                    </p>
                </Link>
                {row.original.description && (
                    <p className="text-gray-500 text-sm">
                        {row.original.description.split(/(\s+)/).map((part, i) => {
                            const urlMatch = part.match(/^(https?:\/\/[^\s]+)$/);
                            if (urlMatch) {
                                return (
                                    <a
                                        key={i}
                                        href={urlMatch[1]}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        className="text-blue-500 hover:underline"
                                        onClick={(e) => e.stopPropagation()}
                                    >
                                        {urlMatch[1]}
                                    </a>
                                );
                            }
                            return part;
                        })}
                    </p>
                )}
            </div>
        }
    },
    {
        id: "project",
        accessorKey: "project",
        header: "Project",
        cell: ({ row }) => {
            return row.original.project ? <ProjectTag projectId={row.original.project.id} asLink /> : "-"
        }
    },
    {
        id: "due_at_utc",
        accessorKey: "due_at_utc",
        header: "Due Date",
        cell: ({ row }) => {
            return <DueDateColumn dateString={row.original.due_at_utc} taskId={row.original.id} task={row.original} />
        }
    },
    {
        id: "deadline_at_utc",
        accessorKey: "deadline_at_utc",
        header: "Deadline",
        cell: ({ row }) => {
            return <DueDateColumn dateString={row.original.deadline_at_utc} taskId={row.original.id} task={row.original} />
        }
    },
    {
        id: "actions",
        cell: ({ row }) => {
            const task = row.original
            return (
                <Popover>
                    <PopoverTrigger asChild>
                        <Button variant="outline" size="xs"><Ellipsis /></Button>
                    </PopoverTrigger>
                    <PopoverContent>
                        <Command>
                            <CommandGroup>
                                <CommandItem>
                                    <EditTaskDialog task={task} />
                                </CommandItem>
                            </CommandGroup>
                        </Command>
                    </PopoverContent>
                </Popover >
            )
        }
    }
]

interface TasksTableProps {
    tasks: Task[]
    hiddenColumns: string[]
    showHeaders?: boolean
}

const TasksTable: React.FC<TasksTableProps> = ({ tasks, hiddenColumns, showHeaders = true }) => {

    // filter out hidden columns
    const filteredColumns = columns.filter((column) => {
        return !hiddenColumns.includes(column.id as string);
    })

    return (
        <DataTable data={tasks} columns={filteredColumns} showHeaders={showHeaders} />
    )
}

export default TasksTable;

interface DueDateColumnProps {
    dateString: string | null,
    taskId: string,
    task: Task,
}

const DueDateColumn: React.FC<DueDateColumnProps> = ({ dateString, taskId, task }) => {
    const [date, setDate] = React.useState<Date | undefined>(dateString ? new Date(dateString) : undefined)
    const [open, setOpen] = React.useState(false)

    const updateDueDateMutation = useMutation({
        mutationFn: async function (newDate: Date | undefined) {
            let res = await invoke_tauri_command('update_task_command', {
                taskId: taskId,
                title: task.title,
                description: task.description || '',
                dueDate: newDate?.toISOString(),
                deadline: task.deadline_at_utc,
                projectId: task.project?.id
            });
            return res
        },
        onSuccess: () => {
            toast.success(`Due date updated to ${date ? format(date, "MMM d") : "none"}`)
            setOpen(false)
        },
    });

    const handleDateChange = (newDate: Date | undefined) => {
        setDate(newDate)
        updateDueDateMutation.mutateAsync(newDate)
    }

    const setQuickDate = (days: number) => {
        const newDate = new Date()
        newDate.setHours(0, 0, 0, 0)
        newDate.setDate(newDate.getDate() + days)
        handleDateChange(newDate)
    }

    const setNextSunday = () => {
        const today = new Date()
        today.setHours(0, 0, 0, 0)
        const nextSunday = startOfWeek(addWeeks(today, 1), { weekStartsOn: 0 })
        handleDateChange(nextSunday)
    }

    return (
        <Popover open={open} onOpenChange={setOpen}>
            <PopoverTrigger asChild>
                <Button variant="ghost" size="sm">
                    {date ? format(date, "MMM d") : "-"}
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-auto p-0" align="start">
                <div className="flex flex-col gap-2 p-2">
                    <div className="flex flex-wrap gap-1">
                        <Button variant="outline" size="sm" onClick={() => setQuickDate(0)}>Today</Button>
                        <Button variant="outline" size="sm" onClick={() => setQuickDate(1)}>Tomorrow</Button>
                        <Button variant="outline" size="sm" onClick={setNextSunday}>Next Sunday</Button>
                    </div>
                    <Calendar
                        mode="single"
                        selected={date}
                        onSelect={handleDateChange}
                        initialFocus
                    />
                </div>
            </PopoverContent>
        </Popover>
    )
}

function ParentTaskLabel({ parentTaskId }: { parentTaskId: string | null }) {
    if (parentTaskId !== null) {
        const query = useQuery({
            queryKey: ['tasks', parentTaskId],
            queryFn: async ({ queryKey }) => {
                const res = await invoke_tauri_command('load_task_by_id_command', { taskId: queryKey[1] })
                return res
            },
        })

        if (query.data) {
            return (
                <p className="text-gray-500 text-xs hover:underline">
                    <Link to="/tasks/$taskId" params={{ taskId: query.data.id }}>
                        {query.data.title}
                    </Link>
                </p>
            )
        }
    }

    return (<></>);
}