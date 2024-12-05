const GlobalTaskForm: React.FC = () => {
    return (
        <p>
            <NewTaskForm />
        </p>
    )
}

export default GlobalTaskForm;

import React from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useForm } from '@tanstack/react-form'
import {
    useQuery,
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { DatePicker } from '../../components/datepicker';
import { Combobox } from '@/components/projects-combobox';
import { invoke_tauri_command } from '@/lib/utils';
import { useParams } from "@tanstack/react-router";


const NewTaskForm: React.FC = () => {
    const queryClient = useQueryClient()

    const { projectId } = useParams({ strict: false })

    const projectListQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            return await invoke_tauri_command('load_projects_command', {})
        }
    })

    const mutation = useMutation({
        mutationFn: async function (value: { title: string, description: string, dueDate: Date | undefined, projectId: string | undefined }) {
            console.debug("Due Date: ", value.dueDate?.toISOString())
            let res = await invoke_tauri_command('save_task_command', { title: value.title, description: value.description, dueDate: value.dueDate, projectId: value.projectId });
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
            projectId: projectId,
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
