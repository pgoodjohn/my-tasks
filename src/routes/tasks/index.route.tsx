import { createFileRoute } from '@tanstack/react-router'
import { RouteComponent } from '@/features/tasks'

export const Route = createFileRoute('/tasks/')({
    component: RouteComponent,
})