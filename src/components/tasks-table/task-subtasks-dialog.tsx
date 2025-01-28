import { Task } from "@/types";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useForm } from '@tanstack/react-form'
import {
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { DatePicker } from '@/components/datepicker';
import { toast } from "sonner";
import { DialogDescription } from "@radix-ui/react-dialog";
import { Separator } from "../ui/separator";
import React, { useState } from 'react';
import {
    useQuery,
} from '@tanstack/react-query'
import { Checkbox } from '../../components/ui/checkbox';
import { invoke_tauri_command } from '@/lib/utils';
import TasksTable from '@/components/tasks-table';

interface TaskSubtasksDialogProps {
    task: Task
}

export function TaskSubtasksDialog({ task }: TaskSubtasksDialogProps) {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button className="w-full text-left" variant="ghost" size="xs">Subtasks</Button>
            </DialogTrigger>
            <DialogContent className="min-w-[720px]">
                <DialogHeader>
                    <DialogTitle>{task.title}</DialogTitle>
                    <DialogDescription>
                        Manage subtasks
                    </DialogDescription>
                </DialogHeader>
                <Separator />
                <CreateSubtaskForm parentTask={task} />
                <Separator />
                <SubTasksTable task={task} />
            </DialogContent>
        </Dialog>
    )
}

function SubTasksTable({ task }: TaskSubtasksDialogProps) {
    const [showCompleted, setShowCompleted] = useState(false)

    const taskListQuery = useQuery({
        queryKey: ['tasks', task.id, 'subtask', showCompleted],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_subtasks_for_task_command', { parentTaskId: task.id, includeCompleted: showCompleted })
            return data
        }
    })

    if (taskListQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (taskListQuery.isError) {
        return <div>Error loading tasks</div>
    }

    return (
        <div className='py-2 max-h-full'>
            <div className="flex space-x-2 pb-4">
                <Checkbox id="show-completed" checked={showCompleted} onCheckedChange={() => setShowCompleted(!showCompleted)} />
                <label
                    htmlFor="show-completed"
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                >
                    Show Completed
                </label>
            </div>
            {taskListQuery.data ? <TasksTable tasks={taskListQuery.data} hiddenColumns={[]} /> : <div>No Data</div>}
        </div>
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