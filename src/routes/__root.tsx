import * as React from 'react'
import { Link, Outlet, createRootRoute } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { SidebarTrigger } from '@/components/ui/sidebar'
import { AppSidebar } from '@/components/app-sidebar'

export const Route = createRootRoute({
    component: RootComponent,
})

function RootComponent() {
    return (
        <React.Fragment>
            <div className='flex'>
                <AppSidebar />
                <div className='w-full'>
                    <div className='flex items-center'>
                        <SidebarTrigger />
                        <p>Breadcrumbs</p>
                    </div>
                    <Outlet />
                </div>
            </div>
        </React.Fragment>
    )
}
