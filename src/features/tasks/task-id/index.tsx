import { CreateSubtaskForm } from "@/components/create-subtask-form";
import { SubtasksTable } from "@/components/subtasks-table";
import { Separator } from "@/components/ui/separator";
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

    if (!taskQuery.data) {
        return <></>
    }

    return (
        <div>
            <div className="flex">
                <p className="text-lg">{taskQuery.data.title}</p>
                <div className="flex-grow" />
                <p>{taskQuery.data.due_at_utc ? (new Date(taskQuery.data.due_at_utc).toDateString()) : "-"}</p>
                <p>{taskQuery.data.deadline_at_utc ? (new Date(taskQuery.data.deadline_at_utc).toDateString()) : "-"}</p>
            </div>
            <p>{taskQuery.data.description}</p>
            <Separator className="my-2" />
            <p>Subtasks</p>
            <CreateSubtaskForm parentTask={taskQuery.data} />
            <SubtasksTable task={taskQuery.data} />
        </div>
    )
}