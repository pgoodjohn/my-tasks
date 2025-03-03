import React from 'react';
import {
    useQuery,
} from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils';
import TasksTable from '@/components/tasks-table';

const Inbox: React.FC = () => {
    return (
        <div className='overflow-auto max-h-full'>
            <div className='pt-2'>
                <InboxTaskList />
            </div>
        </div>
    )
}

const InboxTaskList: React.FC = () => {
    const taskListQuery = useQuery({
        queryKey: ['tasks', 'inbox'],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_tasks_inbox_command', {})
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
            {taskListQuery.data ? <TasksTable tasks={taskListQuery.data} hiddenColumns={[]} showHeaders={false} /> : <div>No Data</div>}
        </div>
    )
}

export default Inbox