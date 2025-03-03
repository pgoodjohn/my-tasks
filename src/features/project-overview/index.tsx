import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import TasksTable from '@/components/tasks-table'
import { Checkbox } from '@/components/ui/checkbox'
import EditProjectDialog from '@/features/projects/edit-project-dialog'
import { FavoriteProjectButton } from '@/components/favorite-project-button'

interface IndexProps {
    projectID: string
}

function Index({ projectID }: IndexProps) {

    const [showCompleted, setShowCompleted] = useState(false)

    const projectDetailQuery = useQuery({
        queryKey: ['tasks', 'projects', projectID, showCompleted],
        queryFn: async () => {
            return await invoke_tauri_command('load_project_details_command', { projectId: projectID, includeCompletedTasks: showCompleted })
        }
    })

    if (projectDetailQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (projectDetailQuery.isError) {
        return <div>Error</div>
    }

    if (!projectDetailQuery.data) {
        return <div>Not found</div>
    }

    return (
        <div>
            <div className='container flex items-center'>
                <p className='text-xl'>{projectDetailQuery.data.project.emoji} {projectDetailQuery.data.project.title}</p>
                <div className='flex-grow' />
                <div className='flex items-center gap-2'>
                    <FavoriteProjectButton project={projectDetailQuery.data.project} />
                    <EditProjectDialog project={projectDetailQuery.data.project} />
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
                <TasksTable tasks={projectDetailQuery.data.tasks} hiddenColumns={["project"]} />
            </div>
        </div>
    )
}

export default Index
