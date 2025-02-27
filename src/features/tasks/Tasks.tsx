import React, { useState } from 'react';
import { Checkbox } from '../../components/ui/checkbox';
import TasksTable from '@/components/tasks-table';
import { useTasks } from '@/hooks/use-tasks';

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
    const tasks = useTasks(showCompleted)

    if (tasks.isLoading) {
        return <div>Loading...</div>
    }

    if (tasks.isError) {
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
            {tasks.data ? <TasksTable tasks={tasks.data} hiddenColumns={[]} /> : <div>No Data</div>}
        </div>
    )
}