import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import TasksTable from '@/components/tasks-table'
import { Checkbox } from '@/components/ui/checkbox'
import { FavoriteProjectButton } from '@/components/favorite-project-button'
import EditProjectDialog from '@/features/projects/edit-project-dialog'
import { useState } from 'react'

interface IndexProps {
    projectID: string
}

function Index({ projectID }: IndexProps) {
    const [showCompleted, setShowCompleted] = useState(false);

    const projectQuery = useQuery({
        queryKey: ['projects', projectID],
        queryFn: async () => {
            return await invoke_tauri_command('load_projects_command', { showArchivedProjects: false })
                .then(projects => projects.find((p: any) => p.id === projectID))
        }
    })

    const tasksQuery = useQuery({
        queryKey: ['tasks', projectID, showCompleted],
        queryFn: async () => {
            return await invoke_tauri_command('load_tasks_by_project_command', { projectId: projectID, includeCompleted: showCompleted })
        }
    })

    if (projectQuery.isLoading || tasksQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (projectQuery.isError || tasksQuery.isError) {
        return <div>Error</div>
    }

    if (!projectQuery.data) {
        return <div>Project not found</div>
    }

    return (
        <div>
            <div className='container flex items-center'>
                <p className='text-xl'>{projectQuery.data.emoji} {projectQuery.data.title}</p>
                <div className='flex-grow' />
                <div className='flex items-center gap-2'>
                    <FavoriteProjectButton project={projectQuery.data} />
                    <EditProjectDialog project={projectQuery.data} />
                </div>
            </div>
            <div className='pt-2'>
                <div className="flex space-x-2 pb-4">
                    <Checkbox id="show-completed" checked={showCompleted} onCheckedChange={() => setShowCompleted(!showCompleted)} />
                    <label
                        htmlFor="show-completed"
                        className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                        Show Completed
                    </label>
                </div>
                <TasksTable tasks={tasksQuery.data || []} hiddenColumns={["project"]} />
            </div>
        </div>
    )
}

export default Index
