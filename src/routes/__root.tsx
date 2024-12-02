import * as React from 'react'
import { Link, Outlet, createRootRoute } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'

export const Route = createRootRoute({
    component: RootComponent,
})

function RootComponent() {
    return (
        <React.Fragment>
            <div className='flex w-min-screen'>
                <div className='flex flex-col'>
                    <Link to='/'>Home</Link>
                    <Link to='/projects'>Projects</Link>
                    <TanStackRouterDevtools />
                </div>
                <div className='w-full'>
                    <Outlet />
                </div>
            </div>
        </React.Fragment>
    )
}
