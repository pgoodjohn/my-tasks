import { createFileRoute } from '@tanstack/react-router'
import { RouteComponent } from '@/features/statistics/route'

export const Route = createFileRoute('/statistics')({
    component: RouteComponent,
})