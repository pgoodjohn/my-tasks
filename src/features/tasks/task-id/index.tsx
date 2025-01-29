import { SubtasksTable } from "@/components/subtasks-table";
import { invoke_tauri_command } from "@/lib/utils";
import { Route } from "@/routes/tasks/$taskId.route"
import { useQuery } from "@tanstack/react-query"

export function RouteComponent() {

    const { taskId } = Route.useParams()

    const taskQuery = useQuery({
        queryKey: ["tasks", taskId],
        queryFn: async ({ queryKey }) => {
            let taskId = queryKey[1];

            return invoke_tauri_command("load_task_by_id_command", { taskId: taskId })
        }
    })

    if (taskQuery.data) {
        console.log(taskQuery.data)
    }

    return (
        <div>
            Task Id: {taskId}
            {taskQuery.data && (
                <SubtasksTable task={taskQuery.data} />
            )}
        </div>
    )
}