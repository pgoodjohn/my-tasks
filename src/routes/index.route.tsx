import ContributionsCalendar from '@/components/contributions-calendar'
import Tasks from '@/features/index/Tasks'
import DueToday from '@/features/index/due-today'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <div>
      <DueToday />
      <Tasks />
      <ContributionsCalendar />
    </div>
  )
}
