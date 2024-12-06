import React from 'react'
import { SidebarTrigger } from './ui/sidebar'
import GlobalTaskForm from './global-task-form'
import { Separator } from './ui/separator'

interface AppContainerProps {
    children: React.ReactNode
}

const AppContainer: React.FC<AppContainerProps> = ({ children }) => {
    return (
        <div className='flex flex-col h-full'>
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
            <div className="w-full flex-grow overflow-auto">
                <div className='p-4'>
                    {children}
                </div>
            </div>
        </div>
    )
}

export default AppContainer