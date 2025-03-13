import { createFileRoute } from '@tanstack/react-router'
import Index from '@/features/project-overview'

export const Route = createFileRoute('/projects/$projectId')({
    component: RouteComponent,
})

function RouteComponent() {
    const { projectId } = Route.useParams() as { projectId: string }

    return (
        <Index projectID={projectId} />
    )
}
