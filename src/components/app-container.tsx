import React from 'react'
import { SidebarTrigger } from './ui/sidebar'
import GlobalTaskForm from './global-task-form'
import { Separator } from './ui/separator'

interface AppContainerProps {
    children: React.ReactNode
}

const AppContainer: React.FC<AppContainerProps> = ({ children }) => {
    return (
        <div>
            <div className='top-0'>
                <div className='flex items-center p-2'>
                    <SidebarTrigger />
                    <p>🍞 Breadcrumbs 🍞</p>
                </div>
                <Separator />
                <div className='p-2'>
                    <GlobalTaskForm />
                </div>
                <Separator />
            </div>
            <div className="w-full max-h-screen p-4 overflow-scroll">
                {children}
            </div>
        </div>
    )
}

export default AppContainer