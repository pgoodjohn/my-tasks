import React, { useEffect } from 'react';
import { useForm } from '@tanstack/react-form'
import {
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { useParams } from "@tanstack/react-router";
import { toast } from 'sonner';
import { DatePicker } from '../../components/datepicker';
import { ProjectsPicker } from '../projects-picker';
import { invoke_tauri_command } from '@/lib/utils';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

const GlobalTaskForm: React.FC = () => {
    return (
        <NewTaskForm />
    )
}

export default GlobalTaskForm;


const NewTaskForm: React.FC = () => {
    const queryClient = useQueryClient()

    const { projectId } = useParams({ strict: false })

    const mutation = useMutation({
        mutationFn: async function (value: { title: string, description: string, dueDate: Date | undefined, projectId: string | undefined }) {
            const res = await invoke_tauri_command('create_task_command', { title: value.title, description: value.description, dueDate: value.dueDate, projectId: value.projectId });
            return res
        },
        onSuccess: () => {
            // Invalidate and refetch
            toast.success(`Task "${newTaskForm.getFieldValue("title")}" created`)
            newTaskForm.reset()
            queryClient.invalidateQueries({ queryKey: ['tasks'] })
        },
        onError: (error) => {
            toast.error(`Error creating task`)
            console.error(error)
        }
    })


    const newTaskForm = useForm({
        defaultValues: {
            title: '',
            description: '',
            projectId: projectId,
            dueDate: undefined,
        },
        onSubmit: async ({ value }) => {
            // Do something with form data
            await mutation.mutateAsync(value)
        },
    })

    useEffect(() => {
        newTaskForm.setFieldValue('projectId', projectId);
    }, [projectId])

    return (
        <div className='py-2 w-full'>
            <form
                className='flex w-full items-center space-x-2'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    newTaskForm.handleSubmit();
                }}
            >
                <div className='flex space-x-2'>
                    <newTaskForm.Field
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
                    <newTaskForm.Field
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
                    <newTaskForm.Field
                        name="projectId"
                        children={(field) => {
                            return (
                                <ProjectsPicker modal={true} selectedValue={field.state.value} onChange={field.handleChange} />
                            )
                        }} />
                    <newTaskForm.Field
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
