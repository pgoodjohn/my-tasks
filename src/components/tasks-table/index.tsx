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
import { Badge } from '../ui/badge';
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


const columns: ColumnDef<Task>[] = [
    {
        id: "completed_at_utc",
        accessorKey: "completed_at_utc",
        header: () => <div className="p-0" />,
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
                    checked={row.getValue("completed_at_utc") != null}
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
        cell: ({ row }) => {
            return <div className='flex flex-col'>
                {row.original.parent_task_id && <ParentTaskLabel parentTaskId={row.original.parent_task_id} />}
                <Link
                    to="/tasks/$taskId" params={{ taskId: row.original.id }}

                >
                    <p className='hover:underline'>
                        {row.original.title}
                    </p>
                </Link>
                {row.original.description && <p className="text-gray-500 text-sm">{row.original.description}</p>}
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
            return <DueDateColumn dateString={row.original.due_at_utc} />
        }
    },
    {
        id: "deadline_at_utc",
        accessorKey: "deadline_at_utc",
        header: "Deadline",
        cell: ({ row }) => {
            return <DueDateColumn dateString={row.original.deadline_at_utc} />
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
}

const TasksTable: React.FC<TasksTableProps> = ({ tasks, hiddenColumns }) => {

    // filter out hidden columns
    const filteredColumns = columns.filter((column) => {
        return !hiddenColumns.includes(column.id as string);
    })

    return (
        <DataTable data={tasks} columns={filteredColumns} />
    )
}

export default TasksTable;

interface DueDateColumnProps {
    dateString: string | null,
}

const DueDateColumn: React.FC<DueDateColumnProps> = ({ dateString }) => {
    if (dateString !== null) {
        const date = new Date(dateString)

        // if date is today, show it in a red Badge
        const today = new Date()
        today.setHours(0, 0, 0, 0)
        if (date.getTime() <= today.getTime()) {
            return <span>
                <Badge variant="destructive">{date.getDate()}/{date.getMonth() + 1}/{date.getFullYear()}</Badge>
            </span>
        }

        return <span>
            <p>{date.getDate()}/{date.getMonth() + 1}/{date.getFullYear()}</p>
        </span>
    }

    return <span>-</span>
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