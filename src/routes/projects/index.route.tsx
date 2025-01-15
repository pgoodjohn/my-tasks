import { createFileRoute } from '@tanstack/react-router'
import Index from '@/features/projects'


export const Route = createFileRoute('/projects/')({
    component: RouteComponent,
})

function RouteComponent() {
    return <Index />
}