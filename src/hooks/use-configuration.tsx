import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';

export function useConfiguration() {
    return useQuery({
        queryKey: ['configuration'],
        queryFn: async () => {
            let configuration = await invoke_tauri_command('load_configuration_command', {});

            console.debug("Loaded Configuration", configuration)

            const projectDetails = await Promise.all(
                configuration.favoriteProjectsUuids.map(async (uuid: string) => {
                    console.debug("Loading favorite project", uuid);
                    return await invoke_tauri_command('load_project_details_command', { projectId: uuid, includeCompletedTasks: false });
                })
            );
            configuration.favoriteProjects = projectDetails;

            console.debug("providing full configuration", configuration)

            return configuration
        }
    });
}