import { createFileRoute } from '@tanstack/react-router'
import Index from '@/features/index'

export const Route = createFileRoute('/')({
  component: RouteComponent,
})

function RouteComponent() {
  return (
    <Index />
  )
}
