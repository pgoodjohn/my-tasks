import React from 'react'
import { ChevronLeft } from "lucide-react"
import { useRouter } from '@tanstack/react-router'
import GlobalTaskForm from './global-task-form'
import { Separator } from './ui/separator'
import { Button } from './ui/button'
import AppBreadcrumb from './app-breadcrumb'
import { SidebarTrigger } from './ui/sidebar'

interface AppContainerProps {
    children: React.ReactNode
}

const AppContainer: React.FC<AppContainerProps> = ({ children }) => {
    return (
        <div className='flex flex-col h-full'>
            <div className='top-0'>
                <div className='flex items-center p-2'>
                    <SidebarTrigger />
                    <BackButton />
                    <AppBreadcrumb />
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

function BackButton() {

    const router = useRouter();

    const handleBack = () => {
        router.history.back();
    }

    return (
        <Button
            variant='ghost'
            className='max-w-[25px]'
            onClick={handleBack}
        >
            <ChevronLeft />
        </Button>
    )
}