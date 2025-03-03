import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import TasksTable from '@/components/tasks-table'

export default function CompletedTasks() {
    const { data: tasks, isLoading } = useQuery({
        queryKey: ['only-completed-tasks'],
        queryFn: async () => {
            return await invoke_tauri_command('load_completed_tasks_command', {})
        }
    })

    if (isLoading) {
        return <div>Loading...</div>
    }

    return (
        <div className="flex flex-col gap-4">
            <div className="flex flex-col gap-4">
                <h1 className="text-2xl font-bold">Completed Tasks</h1>
                <TasksTable tasks={tasks || []} hiddenColumns={[]} />
            </div>
        </div>
    )
} 