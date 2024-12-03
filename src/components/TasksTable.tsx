import React, { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useForm } from '@tanstack/react-form'
import {
    useQuery,
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core';
import { ColumnDef } from "@tanstack/react-table"
import { DataTable } from '@/components/data-table';
import { Checkbox } from '@/components/ui/checkbox';
import { DatePicker } from '@/components/datepicker';
import { Combobox } from '@/components/combobox';
import { Separator } from '@/components/ui/separator';
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { invoke_tauri_command } from '@/lib/utils';
import Tasks from '@/Tasks';
import { Task } from './types';
import EditTaskDialog from '@/components/EditTaskDialog';


const columns: ColumnDef<Task>[] = [
    {
        id: "completed_at_utc",
        accessorKey: "completed_at_utc",
        header: () => <div className="p-0" />,
        cell: ({ row }) => {
            const queryClient = useQueryClient()

            const markCompleteMutation = useMutation({
                mutationFn: async function () {
                    console.debug("Marking update", row.getValue("id"))
                    let res = await invoke('complete_task_command', { taskId: row.original.id })
                    console.debug("Complete Rust Returned", res)
                    return res
                },
                onSuccess: () => {
                    // Invalidate and refetch
                    queryClient.invalidateQueries({ queryKey: ['todos'] })

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
        accessorKey: "title",
        header: "Title",
    },
    {
        accessorKey: "description",
        header: "Description",
    },
    {
        accessorKey: "project",
        header: "Project",
        cell: ({ row }) => {
            console.log(row)
            return row.original.project ? row.original.project.title : "-"
        }
    },
    {
        accessorKey: "due_at_utc",
        header: "Due Date",
        cell: ({ row }) => {
            return row.getValue("due_at_utc") ? new Date(row.getValue("due_at_utc")).toLocaleDateString() : "-"
        }
    },
    {
        id: "actions",
        cell: ({ row }) => {
            const task = row.original

            return (
                <div>
                    <EditTaskDialog task={task} />
                </div>
            )
        }
    }
]

interface TasksTableProps {
    tasks: Task[]
}

const TasksTable: React.FC<TasksTableProps> = ({ tasks }) => {
    return (
        <DataTable data={tasks} columns={columns} />
    )
}

export default TasksTable;