import { RouteComponent } from '@/features/tasks/task-id'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/tasks/$taskId')({
    component: RouteComponent,
})
