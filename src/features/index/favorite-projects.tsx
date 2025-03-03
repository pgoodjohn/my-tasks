import { useFavoriteProjects } from "@/hooks/use-favorite-projects"
import { useTasks } from "@/hooks/use-tasks"
import { Project } from "@/types"
import { ChevronDown, ChevronRight } from "lucide-react"
import { useState } from "react"
import { Link } from "@tanstack/react-router"
import TasksTable from "@/components/tasks-table"

function FavoriteProjects() {
    const { data: projects, isLoading: isLoadingProjects } = useFavoriteProjects()
    const { data: tasks, isLoading: isLoadingTasks } = useTasks(false)
    const [isExpanded, setIsExpanded] = useState(true)

    if (isLoadingProjects || isLoadingTasks || !projects?.length) {
        return null
    }

    const getProjectTasks = (projectId: string) => {
        return tasks?.filter(task => task.project?.id === projectId).slice(0, 5) || []
    }

    return (
        <div className="space-y-2">
            <button
                onClick={() => setIsExpanded(!isExpanded)}
                className="flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-foreground"
            >
                {isExpanded ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
                Favorite Projects
            </button>

            {isExpanded && (
                <div className="space-y-4">
                    {projects.map((project: Project) => {
                        const projectTasks = getProjectTasks(project.id)
                        return (
                            <div key={project.id}>
                                <div className="flex items-center justify-between">
                                    <Link
                                        to="/projects/$projectId"
                                        params={{ projectId: project.id }}
                                        className="flex items-center gap-2 font-medium hover:underline"
                                    >
                                        {project.emoji} {project.title}
                                    </Link>
                                </div>
                                {projectTasks.length > 0 ? (
                                    <div className="mt-2">
                                        <TasksTable
                                            tasks={projectTasks}
                                            hiddenColumns={["project", "deadline_at_utc", "actions"]}
                                            showHeaders={false}
                                        />
                                    </div>
                                ) : (
                                    <div className="mt-2 text-sm text-muted-foreground">
                                        No tasks to show
                                    </div>
                                )}
                            </div>
                        )
                    })}
                </div>
            )}
        </div>
    )
}

export default FavoriteProjects 