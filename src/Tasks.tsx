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
import { DataTable } from './components/data-table';
import { Checkbox } from './components/ui/checkbox';
import { DatePicker } from './components/datepicker';
import Projects from './Projects';
import { Combobox } from './components/combobox';
import { Separator } from './components/ui/separator';

const Tasks: React.FC = () => {

    return (
        <div className='max-h-screen p-8'>
            <div className='flex'>
                <h1 className='text-xl'>Todo List</h1>
                <div className='flex-grow' />
                <Projects />
            </div>
            <div className='pt-2'>
                <div className='flex items-center space-y-2'>
                    <NewTaskForm />
                </div>
                <Separator className='my-4' />
                <TasksList />
            </div>
        </div>
    );
};

export default Tasks;

const TasksList: React.FC = () => {

    const [showCompleted, setShowCompleted] = useState(false)

    const todosListQuery = useQuery({
        queryKey: ['todos', showCompleted],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_tasks_command', { includeCompleted: showCompleted })
            return data
        }
    })

    if (todosListQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (todosListQuery.isError) {
        return <div>Error loading tasks</div>
    }

    if (todosListQuery.data) {
        console.debug("Loaded Data", todosListQuery.data)
    }

    return (
        <div className='py-2'>
            <div className="flex space-x-2 pb-4">
                <Checkbox id="show-completed" checked={showCompleted} onCheckedChange={() => setShowCompleted(!showCompleted)} />
                <label
                    htmlFor="show-completed"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                    Show Completed
                </label>
            </div>
            {todosListQuery.data ? <DataTable data={todosListQuery.data} columns={columns} /> : <div>No Data</div>}
        </div>
    )
}

type Task = {
    id: string
    title: string
    description: string | null
    project: { id: string, title: string } | null
    due_at_utc: string | null
    created_at_utc: string
    updated_at_utc: string
    completed_at_utc: string | null
}

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

import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { invoke_tauri_command } from './lib/utils';


interface EditTaskDialogProps {
    task: Task;
}

const EditTaskDialog: React.FC<EditTaskDialogProps> = ({ task }) => {

    const [open, setOpen] = useState(false)

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger><Button>Edit</Button></DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Edit Task</DialogTitle>
                </DialogHeader>
                <EditTaskForm task={task} onSuccess={setOpen} />
            </DialogContent>
        </Dialog>
    )
}

interface EditTaskFormProps {
    task: Task;
    onSuccess: (open: boolean) => void;
}

const EditTaskForm: React.FC<EditTaskFormProps> = ({ task, onSuccess }) => {
    const queryClient = useQueryClient()

    const todoForm = useForm({
        defaultValues: {
            id: task.id,
            title: task.title,
            description: task.description || '',
            projectId: task.project?.id || undefined,
            dueDate: task.due_at_utc ? new Date(task.due_at_utc) : undefined,
        },
        onSubmit: async ({ value }) => {
            // Do something with form data
            console.log(value)
            await mutation.mutateAsync(value)
        },
    })

    const projectListQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            let res = await invoke('load_projects_command');
            console.debug("Rust Return", res)
            let data = JSON.parse(res as string)
            return data
        }
    })

    const mutation = useMutation({
        mutationFn: async function (value: { id: string, title: string, description: string, dueDate: Date | undefined, projectId: string | undefined }) {
            console.debug("Due Date: ", value.dueDate?.toISOString())
            let res = await invoke('update_task_command', { taskId: value.id, title: value.title, description: value.description, dueDate: value.dueDate, projectId: value.projectId });
            console.debug("Save Rust Returned", res)
            return res
        },
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ['todos'] })
            todoForm.reset()
            onSuccess(false)
        },
    })

    return (
        <div className='py-2 w-full'>
            <form
                className='flex flex-col w-full space-y-4'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    todoForm.handleSubmit();
                }}
            >
                <todoForm.Field
                    name="id"
                    children={(_field) => (
                        <></>
                    )} />
                <todoForm.Field
                    name="title"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Title</label>
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                            />
                        </div>
                    )}
                />
                <todoForm.Field
                    name="description"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Description</label>
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                            />
                        </div>
                    )}
                />
                <todoForm.Field
                    name="projectId"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Project</label>
                            <Combobox values={projectListQuery.data || []} selectedValue={field.state.value} onChange={field.handleChange} />
                        </div>
                    )}
                />
                <todoForm.Field
                    name="dueDate"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Due Date</label>
                            <DatePicker
                                value={field.state.value}
                                onChange={field.handleChange}
                            />
                        </div>
                    )}
                />
                <Button type="submit">Submit</Button>
            </form>
        </div>
    )
}

const NewTaskForm: React.FC = () => {
    const queryClient = useQueryClient()

    const projectListQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            let res = await invoke('load_projects_command');
            console.debug("Rust Return", res)
            let data = JSON.parse(res as string)
            return data
        }
    })

    const mutation = useMutation({
        mutationFn: async function (value: { title: string, description: string, dueDate: Date | undefined, projectId: string | undefined }) {
            console.debug("Due Date: ", value.dueDate?.toISOString())
            let res = await invoke('save_task_command', { title: value.title, description: value.description, dueDate: value.dueDate, projectId: value.projectId });
            console.debug("Save Rust Returned", res)
            return res
        },
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ['todos'] })
            todoForm.reset()
        },
    })


    const todoForm = useForm({
        defaultValues: {
            title: '',
            description: '',
            projectId: undefined,
            dueDate: undefined,
        },
        onSubmit: async ({ value }) => {
            // Do something with form data
            console.log(value)
            await mutation.mutateAsync(value)
        },
    })

    return (
        <div className='py-2 w-full'>
            <form
                className='flex w-full items-center space-x-2'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    todoForm.handleSubmit();
                }}
            >
                <div className='flex space-x-2'>
                    <todoForm.Field
                        name="title"
                        children={(field) => (
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                            />
                        )}
                    />
                    <todoForm.Field
                        name="description"
                        children={(field) => (
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                            />
                        )}
                    />
                    <todoForm.Field
                        name="projectId"
                        children={(field) => {
                            return (
                                <Combobox values={projectListQuery.data || []} selectedValue={field.state.value} onChange={field.handleChange} />
                            )
                        }} />
                    <todoForm.Field
                        name="dueDate"
                        children={(field) => (
                            <DatePicker
                                value={field.state.value}
                                onChange={field.handleChange}
                            />
                        )}
                    />
                </div>
                <Button type="submit">Submit</Button>
            </form>
        </div>
    )
}