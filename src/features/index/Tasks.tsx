import React, { useState } from 'react';
import {
    useQuery,
} from '@tanstack/react-query'
import { Checkbox } from '../../components/ui/checkbox';
import ProjectsSheet from './ProjectsSheet';
import { invoke_tauri_command } from '@/lib/utils';
import TasksTable from '@/components/tasks-table';

const Tasks: React.FC = () => {

    return (
        <div>
            <div className='flex'>
                <h1 className='text-xl'>Todo List</h1>
                <div className='flex-grow' />
                <ProjectsSheet />
            </div>
            <div className='pt-2'>
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
            {todosListQuery.data ? <TasksTable tasks={todosListQuery.data} hiddenColumns={[]} /> : <div>No Data</div>}
        </div>
    )
}