import React, { useState } from 'react';
import { useForm } from '@tanstack/react-form'
import {
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { toast } from "sonner";
import { ProjectsPicker } from "../projects-picker";
import type { Task } from "@/types";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { DatePicker } from '@/components/datepicker';
import { invoke_tauri_command } from "@/lib/utils";

interface EditTaskDialogProps {
    task: Task;
}

const EditTaskDialog: React.FC<EditTaskDialogProps> = ({ task }) => {

    const [open, setOpen] = useState(false)

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button className="w-full text-left" variant="ghost" size="xs">Edit</Button>
            </DialogTrigger>
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

    const editTaskForm = useForm({
        defaultValues: {
            title: task.title,
            description: task.description || '',
            projectId: task.project_id || undefined,
            dueDate: task.due_at_utc ? new Date(task.due_at_utc) : undefined,
        },
        onSubmit: async (values) => {
            try {
                await updateTaskMutation.mutateAsync({
                    id: task.id,
                    title: values.title,
                    description: values.description,
                    dueDate: values.dueDate?.toISOString(),
                    projectId: values.projectId,
                });
                onSuccess(false);
            } catch (error) {
                console.error('Failed to update task:', error);
                toast.error('Failed to update task');
            }
        },
    });

    const updateTaskMutation = useMutation({
        mutationFn: async function (value: { id: string, title: string, description: string, dueDate: string | undefined, projectId: string | undefined }) {
            const res = await invoke_tauri_command('update_task_command', {
                taskId: value.id,
                title: value.title,
                description: value.description,
                dueDate: value.dueDate,
                projectId: value.projectId
            });
            return res;
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['tasks'] });
            toast.success(`Task "${editTaskForm.getFieldValue('title')}" was updated`);
            editTaskForm.reset();
        },
        onError: (error: any) => {
            console.error('Update mutation failed:', error);
            toast.error(`Failed to update task: ${error.message}`);
        }
    });

    return (
        <div className='py-2 w-full'>
            <form
                className='flex flex-col w-full space-y-4'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    editTaskForm.handleSubmit();
                }}
            >
                <editTaskForm.Field
                    name="id"
                    children={(_field) => (
                        <></>
                    )} />
                <editTaskForm.Field
                    name="title"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Title</label>
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                                placeholder="Enter task title"
                                onKeyDown={(e) => {
                                    if (e.key === 'Enter') {
                                        e.preventDefault();
                                        editTaskForm.handleSubmit();
                                    }
                                }}
                            />
                        </div>
                    )}
                />
                <editTaskForm.Field
                    name="description"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Description</label>
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                                placeholder="Enter task description"
                                onKeyDown={(e) => {
                                    if (e.key === 'Enter') {
                                        e.preventDefault();
                                        editTaskForm.handleSubmit();
                                    }
                                }}
                            />
                        </div>
                    )}
                />
                <editTaskForm.Field
                    name="projectId"
                    children={(field) => (
                        <div className='flex flex-col'>
                            <label className='text-sm font-medium'>Project</label>
                            <ProjectsPicker modal={true} selectedValue={field.state.value} onChange={field.handleChange} />
                        </div>
                    )}
                />
                <editTaskForm.Field
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
                <div className='flex justify-end space-x-2'>
                    <Button variant="outline" onClick={(_e) => onSuccess(false)}>Cancel</Button>
                    <Button type="submit">Save</Button>
                </div>
            </form>
        </div>
    )
}