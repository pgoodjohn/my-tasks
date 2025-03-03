import { Task } from "@/types"
import { useQuery } from "@tanstack/react-query"
import { invoke_tauri_command } from "@/lib/utils"
import TasksTable from "./tasks-table"

interface SubtasksTableProps {
    task: Task
}

export function SubtasksTable({ task }: SubtasksTableProps) {
    const incompleteTasksQuery = useQuery({
        queryKey: ['tasks', task.id, 'subtask', false],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_subtasks_for_task_command', { parentTaskId: task.id })
            return data
        }
    })

    const completedTasksQuery = useQuery({
        queryKey: ['tasks', task.id, 'completed-subtask'],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_completed_subtasks_for_task_command', { parentTaskId: task.id })
            return data
        }
    })

    if (incompleteTasksQuery.isLoading || completedTasksQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (incompleteTasksQuery.isError || completedTasksQuery.isError) {
        return <div>Error loading tasks</div>
    }

    return (
        <div className='space-y-6'>
            {/* Incomplete Tasks */}
            {incompleteTasksQuery.data && incompleteTasksQuery.data.length > 0 && (
                <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-2">Incomplete</h3>
                    <TasksTable tasks={incompleteTasksQuery.data} hiddenColumns={[]} />
                </div>
            )}

            {/* Completed Tasks */}
            {completedTasksQuery.data && completedTasksQuery.data.length > 0 && (
                <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-2">Completed</h3>
                    <TasksTable tasks={completedTasksQuery.data} hiddenColumns={[]} />
                </div>
            )}
        </div>
    )
}