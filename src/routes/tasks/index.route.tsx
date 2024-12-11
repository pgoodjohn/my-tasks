import { createFileRoute } from '@tanstack/react-router'
import Tasks from '@/features/index/Tasks'

export const Route = createFileRoute('/tasks/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <Tasks />
}
