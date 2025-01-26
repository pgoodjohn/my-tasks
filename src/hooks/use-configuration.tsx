import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';

export function useConfiguration() {
    return useQuery({
        queryKey: ['configuration'],
        queryFn: async () => {
            let configuration = await invoke_tauri_command('load_configuration_command', {});

            return configuration
        }
    });
}