import React, { useState } from 'react';
import { Checkbox } from '../../components/ui/checkbox';
import { ProjectExclusionSheet } from './ProjectExclusionSheet';
import TasksTable from '@/components/tasks-table';
import { useTasks } from '@/hooks/use-tasks';
import { useExcludedProjects } from '@/hooks/use-excluded-projects';

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
    const [excludedProjects, setExcludedProjects] = useExcludedProjects()
    const tasks = useTasks(showCompleted)

    if (tasks.isLoading) {
        return <div>Loading...</div>
    }

    if (tasks.isError) {
        return <div>Error loading tasks</div>
    }

    return (
        <div className='py-2 max-h-full'>
            <div className="flex justify-between items-center pb-4">
                <div className="flex items-center space-x-2">
                    <Checkbox id="show-completed" checked={showCompleted} onCheckedChange={() => setShowCompleted(!showCompleted)} />
                    <label
                        htmlFor="show-completed"
                        className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                        Show Completed
                    </label>
                </div>
                <ProjectExclusionSheet
                    excludedProjects={excludedProjects}
                    onExcludedProjectsChange={setExcludedProjects}
                />
            </div>
            {tasks.data ? (
                <TasksTable
                    tasks={tasks.data.filter(task => task.project_id && !excludedProjects.includes(task.project_id))}
                    hiddenColumns={[]}
                />
            ) : (
                <div>No Data</div>
            )}
        </div>
    )
}