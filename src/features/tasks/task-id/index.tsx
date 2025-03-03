import { useNavigate } from "@tanstack/react-router"
import { useQuery } from "@tanstack/react-query"
import { toast } from "sonner"
import { CalendarIcon } from "lucide-react"
import { CreateSubtaskForm } from "@/components/create-subtask-form";
import { SubtasksTable } from "@/components/subtasks-table";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { invoke_tauri_command } from "@/lib/utils";
import { Route } from "@/routes/tasks/$taskId.route"

export function RouteComponent() {
    const { taskId } = Route.useParams()
    const navigate = useNavigate()

    const taskQuery = useQuery({
        queryKey: ["tasks", taskId],
        queryFn: async ({ queryKey }) => {
            const taskId = queryKey[1];
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
        <div className="space-y-6">
            {/* Header Section */}
            <div className="flex items-center justify-between">
                <div className="space-y-1">
                    <h1 className="text-xl font-semibold">{taskQuery.data.title}</h1>
                    {taskQuery.data.due_at_utc && (
                        <div className="flex items-center gap-2 text-sm text-muted-foreground">
                            <CalendarIcon className="h-4 w-4" />
                            <span>Due {new Date(taskQuery.data.due_at_utc).toLocaleDateString()}</span>
                        </div>
                    )}
                </div>
                <Button variant="outline" onClick={handlePromoteToProject}>
                    Promote to Project
                </Button>
            </div>

            {/* Description */}
            {taskQuery.data.description && (
                <p className="text-muted-foreground whitespace-pre-wrap">{taskQuery.data.description}</p>
            )}

            <Separator />

            {/* Subtasks Section */}
            <div className="space-y-4">
                <CreateSubtaskForm parentTask={taskQuery.data} />
                <SubtasksTable task={taskQuery.data} />
            </div>
        </div>
    )
}