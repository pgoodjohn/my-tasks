import { createFileRoute } from '@tanstack/react-router'
import Index from '@/features/tasks-list'

export const Route = createFileRoute('/tasks/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <Index />
}
