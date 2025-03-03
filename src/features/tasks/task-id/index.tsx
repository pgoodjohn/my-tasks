import { CreateSubtaskForm } from "@/components/create-subtask-form";
import { SubtasksTable } from "@/components/subtasks-table";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { invoke_tauri_command } from "@/lib/utils";
import { Route } from "@/routes/tasks/$taskId.route"
import { useNavigate } from "@tanstack/react-router"
import { useQuery } from "@tanstack/react-query"
import { toast } from "sonner"

export function RouteComponent() {
    const { taskId } = Route.useParams()
    const navigate = useNavigate()

    const taskQuery = useQuery({
        queryKey: ["tasks", taskId],
        queryFn: async ({ queryKey }) => {
            let taskId = queryKey[1];

            return invoke_tauri_command("load_task_by_id_command", { taskId: taskId })
        }
    })

    const handlePromoteToProject = async () => {
        if (!taskId) return;

        try {
            const project = await invoke_tauri_command("promote_task_to_project_command", { taskId });
            toast.success(`Task promoted to project "${project.title}"`);
            navigate({ to: '/projects/$projectId', params: { projectId: project.id } });
        } catch (error) {
            console.error("Failed to promote task to project:", error);
            toast.error("Failed to promote task to project");
        }
    }

    if (!taskQuery.data) {
        return <></>
    }

    return (
        <div>
            <div className="flex items-center gap-4">
                <p className="text-lg">{taskQuery.data.title}</p>
                <div className="flex-grow" />
                <p>{taskQuery.data.due_at_utc ? (new Date(taskQuery.data.due_at_utc).toDateString()) : "-"}</p>
                <p>{taskQuery.data.deadline_at_utc ? (new Date(taskQuery.data.deadline_at_utc).toDateString()) : "-"}</p>
                <Button variant="outline" onClick={handlePromoteToProject}>
                    Promote to Project
                </Button>
            </div>
            <p>{taskQuery.data.description}</p>
            <Separator className="my-2" />
            <p>Subtasks</p>
            <CreateSubtaskForm parentTask={taskQuery.data} />
            <SubtasksTable task={taskQuery.data} />
        </div>
    )
}