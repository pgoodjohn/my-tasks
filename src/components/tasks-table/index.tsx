import React from 'react';
import {
    useMutation,
    useQuery,
    useQueryClient,
} from '@tanstack/react-query'
import { Ellipsis, Pencil, Trash2 } from 'lucide-react';
import { Link } from '@tanstack/react-router'
import { addWeeks, format, startOfWeek } from "date-fns"
import { toast } from "sonner"
import { Button } from '../ui/button';
import type { ColumnDef } from "@tanstack/react-table"
import type { Task } from '@/types';
import { DataTable } from '@/components/data-table';
import { Checkbox } from '@/components/ui/checkbox';
import EditTaskDialog from '@/components/tasks-table/EditTaskDialog';
import ProjectTag from '@/components/project-tag';
import { invoke_tauri_command } from '@/lib/utils';
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import {
    Command,
    CommandGroup,
    CommandItem,
} from "@/components/ui/command"
import { Calendar } from "@/components/ui/calendar"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
} from "@/components/ui/tooltip"


const columns: Array<ColumnDef<Task>> = [
    {
        id: "complete",
        size: 10,
        cell: ({ row }) => {
            const queryClient = useQueryClient()

            const markCompleteMutation = useMutation({
                mutationFn: async function () {
                    await invoke_tauri_command('complete_task_command', { taskId: row.original.id });
                },
                onSuccess: () => {
                    // Invalidate and refetch
                    queryClient.invalidateQueries({ queryKey: ['tasks'] })

                },
            });

            return (
                <div className="flex items-center justify-center px-2">
                    <Checkbox
                        checked={row.original.completed_at_utc != null}
                        onCheckedChange={() => {
                            markCompleteMutation.mutateAsync()
                        }}
                    />
                </div>
            )
        }
    },
    {
        id: "title",
        accessorKey: "title",
        header: "Task",
        size: 400,
        cell: ({ row }) => {
            return <div className='flex-col pl-2'>
                {row.original.parent_task_id && <ParentTaskLabel parentTaskId={row.original.parent_task_id} />}
                <Link
                    to="/tasks/$taskId"
                    params={{ taskId: row.original.id } as any}
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
        header: "Project",
        size: 120,
        cell: ({ row }) => {
            return row.original.project_id ? <ProjectTag projectId={row.original.project_id} asLink /> : "-"
        }
    },
    {
        id: 'due_at_utc',
        header: "Due Date",
        size: 100,
        cell: ({ row }) => {
            return <DueDateColumn dateString={row.original.due_at_utc} taskId={row.original.id} task={row.original} />
            // return <div>{row.original.due_at_utc}</div>
        }
    },
    {
        id: "actions",
        size: 50,
        cell: ({ row }) => {
            const task = row.original
            const queryClient = useQueryClient()
            const [open, setOpen] = React.useState(false)
            const [deleteDialogOpen, setDeleteDialogOpen] = React.useState(false)

            const deleteTaskMutation = useMutation({
                mutationFn: async function () {
                    console.log('Deleting task:', task.id);
                    const res = await invoke_tauri_command('delete_task_command', { taskId: task.id });
                    console.log('Delete response:', res);
                    return res;
                },
                onSuccess: () => {
                    console.log('Delete mutation succeeded');
                    queryClient.invalidateQueries({ queryKey: ['tasks'] })
                    toast.success(`Task "${task.title}" was deleted`)
                    setOpen(false)
                    setDeleteDialogOpen(false)
                },
                onError: (error) => {
                    console.error('Delete mutation failed:', error);
                    toast.error(`Failed to delete task: ${error.message}`)
                }
            });

            return (
                <Popover open={open} onOpenChange={setOpen}>
                    <PopoverTrigger asChild>
                        <Button variant="ghost" size="icon" className="h-8 w-8">
                            <Ellipsis className="h-4 w-4" />
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent className="w-48 p-0" align="end">
                        <Command>
                            <CommandGroup>
                                <CommandItem className="flex items-center gap-2">
                                    <Pencil className="h-4 w-4" />
                                    <EditTaskDialog task={task} />
                                </CommandItem>
                                <Dialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
                                    <DialogTrigger asChild>
                                        <CommandItem
                                            className="flex items-center gap-2 text-red-600"
                                            onSelect={() => setDeleteDialogOpen(true)}
                                        >
                                            <Trash2 className="h-4 w-4" />
                                            Delete Task
                                        </CommandItem>
                                    </DialogTrigger>
                                    <DialogContent>
                                        <DialogHeader>
                                            <DialogTitle>Delete Task</DialogTitle>
                                            <DialogDescription>
                                                Are you sure you want to delete "{task.title}"? This action cannot be undone.
                                            </DialogDescription>
                                        </DialogHeader>
                                        <DialogFooter>
                                            <Button variant="outline" onClick={() => setDeleteDialogOpen(false)}>
                                                Cancel
                                            </Button>
                                            <Button
                                                variant="destructive"
                                                onClick={async () => {
                                                    try {
                                                        await deleteTaskMutation.mutateAsync();
                                                    } catch (error) {
                                                        console.error('Delete failed:', error);
                                                    }
                                                }}
                                            >
                                                Delete
                                            </Button>
                                        </DialogFooter>
                                    </DialogContent>
                                </Dialog>
                            </CommandGroup>
                        </Command>
                    </PopoverContent>
                </Popover>
            )
        }
    }
]

interface TasksTableProps {
    tasks: Array<Task>
    hiddenColumns: Array<string>
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

interface RecurringTaskTooltipProps {
    recurringTask: {
        frequency: string;
        interval: number;
        next_due_at_utc: string;
    } | null;
}

const RecurringTaskTooltip: React.FC<RecurringTaskTooltipProps> = ({ recurringTask }) => {
    if (!recurringTask) return null;

    const getFrequencyText = (frequency: string, interval: number) => {
        if (interval === 1) {
            switch (frequency.toLowerCase()) {
                case 'daily': return 'Every day';
                case 'weekly': return 'Every week';
                case 'monthly': return 'Every month';
                case 'yearly': return 'Every year';
                default: return `Every ${frequency.toLowerCase()}`;
            }
        } else {
            switch (frequency.toLowerCase()) {
                case 'daily': return `Every ${interval} days`;
                case 'weekly': return `Every ${interval} weeks`;
                case 'monthly': return `Every ${interval} months`;
                case 'yearly': return `Every ${interval} years`;
                default: return `Every ${interval} ${frequency.toLowerCase()}s`;
            }
        }
    };

    return (
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger>
                    <svg className="h-3 w-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                        <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                        <path d="M3 3v5h5" />
                    </svg>
                </TooltipTrigger>
                <TooltipContent>
                    <p>{getFrequencyText(recurringTask.frequency, recurringTask.interval)}</p>
                    <p>Next due: {format(new Date(recurringTask.next_due_at_utc), "MMM d, yyyy")}</p>
                </TooltipContent>
            </Tooltip>
        </TooltipProvider>
    );
};

interface DueDateColumnProps {
    dateString: string | null,
    taskId: string,
    task: Task,
}

const DueDateColumn: React.FC<DueDateColumnProps> = ({ dateString, taskId, task }) => {
    const [date, setDate] = React.useState<Date | undefined>(dateString ? new Date(dateString) : undefined)
    const [open, setOpen] = React.useState(false)
    const queryClient = useQueryClient()
    const { data: recurringTask } = useQuery({
        queryKey: ["tasks", { taskId }],
        queryFn: async () => {
            const result = await invoke_tauri_command("get_recurring_task_command", { taskId })
            return result
        }
    })

    React.useEffect(() => {
        setDate(dateString ? new Date(dateString) : undefined);
    }, [dateString]);

    const getDueDateStyle = () => {
        if (!date) return {};

        const today = new Date();
        today.setHours(0, 0, 0, 0);
        const taskDate = new Date(date);
        taskDate.setHours(0, 0, 0, 0);

        if (taskDate < today) {
            return { borderColor: 'rgb(239 68 68)', borderWidth: '1px', borderStyle: 'solid' };
        }
        if (taskDate.getTime() === today.getTime()) {
            return { borderColor: 'rgb(249 115 22)', borderWidth: '1px', borderStyle: 'solid' };
        }
        return {};
    }

    const updateDueDateMutation = useMutation({
        mutationFn: async function (newDate: Date | undefined) {
            const res = await invoke_tauri_command('update_task_command', {
                taskId: taskId,
                title: task.title,
                description: task.description || '',
                dueDate: newDate?.toISOString(),
                projectId: task.project_id
            });
            return res
        },
        onSuccess: () => {
            toast.success(`Due date updated to ${date ? format(date, "MMM d") : "none"}`)
            setOpen(false)
            queryClient.invalidateQueries({ queryKey: ['tasks'] })
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
                <Button
                    variant="ghost"
                    size="xs"
                    style={getDueDateStyle()}
                    className="flex items-center gap-1"
                >
                    {date ? format(date, "MMM d") : "-"}
                    <RecurringTaskTooltip recurringTask={recurringTask} />
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
                    <Link
                        to="/tasks/$taskId"
                        params={{ taskId: query.data.id } as any}
                    >
                        {query.data.title}
                    </Link>
                </p>
            )
        }
    }

    return (<></>);
}