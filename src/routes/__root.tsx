import * as React from 'react'
import { Outlet, createRootRoute } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { SidebarTrigger } from '@/components/ui/sidebar'
import { AppSidebar } from '@/components/app-sidebar'
import { useConfiguration } from '@/hooks/use-configuration'
import AppContainer from '@/components/app-container'
import GlobalTaskForm from '@/components/global-task-form'
import { Separator } from '@/components/ui/separator'

export const Route = createRootRoute({
    component: RootComponent,
})

function RootComponent() {
    const { data } = useConfiguration()

    return (
        <React.Fragment>
            <div className='flex w-full max-w-screen'>
                <AppSidebar />
                <div className='w-full'>
                    <div className='flex items-center p-2'>
                        <SidebarTrigger />
                        <p>🍞 Breadcrumbs 🍞</p>
                    </div>
                    <Separator />
                    <div className='p-2'>
                        <GlobalTaskForm />
                    </div>
                    <Separator />
                    <AppContainer>
                        <Outlet />
                    </AppContainer>
                </div>
            </div>
            {data?.developmentMode && <TanStackRouterDevtools position='bottom-right' />}
        </React.Fragment >
    )
}
