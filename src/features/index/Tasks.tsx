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
import { Checkbox } from '../../components/ui/checkbox';
import { DatePicker } from '../../components/datepicker';
import ProjectsSheet from './ProjectsSheet';
import { Combobox } from '../../components/combobox';
import { Separator } from '../../components/ui/separator';
import { invoke_tauri_command } from '@/lib/utils';
import TasksTable from '@/components/tasks-table';

const Tasks: React.FC = () => {

    return (
        <div className='w-full max-h-screen p-4'>
            <div className='flex'>
                <h1 className='text-xl'>Todo List</h1>
                <div className='flex-grow' />
                <ProjectsSheet />
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
            {todosListQuery.data ? <TasksTable tasks={todosListQuery.data} /> : <div>No Data</div>}
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