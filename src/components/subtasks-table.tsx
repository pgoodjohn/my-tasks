import { Task } from "@/types"
import { useQuery } from "@tanstack/react-query"
import { invoke_tauri_command } from "@/lib/utils"
import { useState } from "react"
import { Checkbox } from "./ui/checkbox"
import TasksTable from "./tasks-table"

interface SubtasksTableProps {
    task: Task
}

export function SubtasksTable({ task }: SubtasksTableProps) {
    const [showCompleted, setShowCompleted] = useState(false)

    const taskListQuery = useQuery({
        queryKey: ['tasks', task.id, 'subtask', showCompleted],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_subtasks_for_task_command', { parentTaskId: task.id, includeCompleted: showCompleted })
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