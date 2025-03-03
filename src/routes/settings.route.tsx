import { createFileRoute } from '@tanstack/react-router'
import { useConfiguration } from '@/hooks/use-configuration'

export const Route = createFileRoute('/settings')({
    component: RouteComponent,
})

function RouteComponent() {

    return <div>
        <p className='text-xl'>Settings</p>
        <SettingsPreview />
    </div>
}

function SettingsPreview() {
    const configuration = useConfiguration();


    if (configuration.isLoading || configuration.isPending) {
        return <></>
    }

    if (configuration.isError) {
        return <p>Error loading configuration</p>
    }


    return (
        <>
            {configuration.data && (
                <div>
                    <p>Configuration</p>
                    <pre>{JSON.stringify(configuration.data)}</pre>
                </div>
            )
            }
        </>
    )
}
