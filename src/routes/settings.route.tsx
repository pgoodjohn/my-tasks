import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/settings')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div>
        <p className='text-xl'>Settings</p>
    </div>
}
