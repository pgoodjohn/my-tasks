import { Task } from "@/types";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
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
import { DatePicker } from '@/components/datepicker';
import { Combobox } from '@/components/projects-combobox';
import { toast } from "sonner";
import { invoke_tauri_command } from "@/lib/utils";

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

export default EditTaskDialog

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
            return invoke_tauri_command('load_projects_command', { showArchivedProjects: false })
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
            toast.success(`Task "${todoForm.getFieldValue("title")}" was updated`)
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
                <div className=''>
                    <Button variant="outline" className='m-2' onClick={(_e) => onSuccess(false)}>Cancel</Button>
                    <Button className="m-2" type="submit">Save</Button>
                </div>
            </form>
        </div>
    )
}