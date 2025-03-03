import { createFileRoute } from '@tanstack/react-router'
import CompletedTasks from '@/features/completed-tasks'

export const Route = createFileRoute('/tasks/completed')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <CompletedTasks />
    )
} 