import { createFileRoute } from '@tanstack/react-router'
import { useConfiguration } from '@/hooks/use-configuration'

export const Route = createFileRoute('/settings')({
    component: RouteComponent,
})

function RouteComponent() {

    return <div>
        <SettingsPreview />
    </div>
}

function SettingsPreview() {
    const configuration = useConfiguration();

    if (configuration.isLoading || configuration.isPending) {
        return <div className="animate-pulse">
            <div className="h-4 bg-gray-200 rounded w-1/4 mb-4"></div>
            <div className="space-y-3">
                <div className="h-4 bg-gray-200 rounded w-3/4"></div>
                <div className="h-4 bg-gray-200 rounded w-1/2"></div>
                <div className="h-4 bg-gray-200 rounded w-2/3"></div>
            </div>
        </div>;
    }

    if (configuration.isError) {
        return <div className="p-4 rounded-lg bg-red-50 border border-red-200">
            <p className="text-red-700">Error loading configuration</p>
        </div>
    }

    if (!configuration.data) return null;

    const { version, developmentMode, configurationPath, dbPath, ollama } = configuration.data;

    return (
        <div className="space-y-6 max-w-2xl">
            <div className="bg-white rounded-lg border border-gray-200 p-4">
                <h3 className="text-lg font-medium text-gray-900 mb-3">General Settings</h3>
                <div className="space-y-2">
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Version</span>
                        <span className="font-mono bg-gray-50 px-2 py-1 rounded text-sm">{version}</span>
                    </div>
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Development Mode</span>
                        <span className={`px-2 py-1 rounded-full text-sm ${developmentMode ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'}`}>
                            {developmentMode ? 'Enabled' : 'Disabled'}
                        </span>
                    </div>
                </div>
            </div>

            <div className="bg-white rounded-lg border border-gray-200 p-4">
                <h3 className="text-lg font-medium text-gray-900 mb-3">File Paths</h3>
                <div className="space-y-2">
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Config File</span>
                        <span className="font-mono bg-gray-50 px-2 py-1 rounded text-sm">{configurationPath}</span>
                    </div>
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Database File</span>
                        <span className="font-mono bg-gray-50 px-2 py-1 rounded text-sm">{dbPath}</span>
                    </div>
                </div>
            </div>

            <div className="bg-white rounded-lg border border-gray-200 p-4">
                <h3 className="text-lg font-medium text-gray-900 mb-3">Ollama Configuration</h3>
                <div className="space-y-2">
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Base URL</span>
                        <span className="font-mono bg-gray-50 px-2 py-1 rounded text-sm">{ollama.base_url}</span>
                    </div>
                    <div className="flex justify-between items-center py-2 border-b border-gray-100">
                        <span className="text-gray-600">Model</span>
                        <span className="font-mono bg-gray-50 px-2 py-1 rounded text-sm">{ollama.model}</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
