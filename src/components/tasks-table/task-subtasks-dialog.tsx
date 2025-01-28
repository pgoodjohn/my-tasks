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
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { DatePicker } from '@/components/datepicker';
import { toast } from "sonner";
import { invoke_tauri_command } from "@/lib/utils";
import { ProjectsPicker } from "../projects-picker";
import { DialogDescription } from "@radix-ui/react-dialog";
import { Separator } from "../ui/separator";

interface TaskSubtasksDialogProps {
    task: Task
}

export function TaskSubtasksDialog({ task }: TaskSubtasksDialogProps) {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button className="w-full text-left" variant="ghost" size="xs">Subtasks</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Subtasks Management</DialogTitle>
                    <DialogDescription>
                        {task.id}
                    </DialogDescription>
                </DialogHeader>
                <Separator />
                <CreateSubtaskForm parentTask={task} />
            </DialogContent>
        </Dialog>
    )
}

interface CreateSubtaskFormProps {
    parentTask: Task
}

function CreateSubtaskForm({ parentTask }: CreateSubtaskFormProps) {

    const queryClient = useQueryClient()

    const newSubtaskMutation = useMutation({
        mutationFn: async ({ value }: { value: { parentTaskId: string, title: string, description: string, dueDate: Date | null } }) => {
            let res = await invoke_tauri_command('create_subtask_for_task_command', { parentTaskId: value.parentTaskId, title: value.title, description: value.description, due_date: value.dueDate })

            return res
        },
        onSuccess: () => {
            toast.success("Subtask created")
            queryClient.invalidateQueries({ queryKey: ['tasks'] })
            newSubtaskForm.reset()
        },
        onError: (error) => {
            console.error(error)
            // Handle the error, e.g., show an error message to the user or take other actions as needed.
            toast.error("Error creating subtask")
        }
    })

    const newSubtaskForm = useForm({
        defaultValues: {
            parentTaskId: parentTask.id,
            title: "",
            description: "",
            dueDate: null,
        },
        onSubmit: async (data) => {
            console.log("Submitting form", data)
            newSubtaskMutation.mutateAsync(data)
        }
    })

    return (
        <div>
            <form
                className='flex w-full items-center space-x-2'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    newSubtaskForm.handleSubmit();
                }}
            >
                <div className='flex space-x-2'>
                    <newSubtaskForm.Field
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
                    <newSubtaskForm.Field
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