import { createFileRoute } from '@tanstack/react-router'
import { RouteComponent } from '@/features/tasks/task-id'

export const Route = createFileRoute('/tasks/$taskId')({
    component: RouteComponent,
})
