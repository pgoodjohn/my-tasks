import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { invoke_tauri_command } from '@/lib/utils'
import TasksTable from '@/components/tasks-table'

export const Route = createFileRoute('/projects/$projectId')({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams()

    const projectDetailQuery = useQuery({
        queryKey: ['todos', 'projects', projectId],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_project_details_command', { projectId })
            return data
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

    console.debug('projectDetailQuery.data', projectDetailQuery.data)

    return (
        <div>
            <p>Hello "/projects/{projectId}"!</p>
            <TasksTable tasks={projectDetailQuery.data.tasks} />
        </div>

    )
}
