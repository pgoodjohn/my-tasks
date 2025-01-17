import React, { useState } from 'react';
import {
    useQuery,
} from '@tanstack/react-query'
import { Checkbox } from '../../components/ui/checkbox';
import { invoke_tauri_command } from '@/lib/utils';
import TasksTable from '@/components/tasks-table';

const Tasks: React.FC = () => {

    return (
        <div className='overflow-auto max-h-full'>
            <div className='pt-2'>
                <TasksList />
            </div>
        </div>
    );
};

export default Tasks;

const TasksList: React.FC = () => {

    const [showCompleted, setShowCompleted] = useState(false)

    const taskListQuery = useQuery({
        queryKey: ['tasks', showCompleted],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_tasks_command', { includeCompleted: showCompleted })
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