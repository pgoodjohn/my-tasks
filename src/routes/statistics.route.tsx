import { createFileRoute } from '@tanstack/react-router'
import ContributionsCalendar from '@/components/contributions-calendar'

export const Route = createFileRoute('/statistics')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div>
        <ContributionsCalendar />
    </div>
}
