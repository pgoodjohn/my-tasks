import { useConfiguration } from '@/hooks/use-configuration'
import { createFileRoute } from '@tanstack/react-router'

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

    console.log("Loaded configuration data", configuration.data)


    return (
        <>
            {configuration.data && (
                <pre>{JSON.stringify(configuration.data)}</pre>
            )
            }
        </>
    )
}
