import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';
import { Project } from '@/types';

export function useFavoriteProjects(): { data: Project[] | undefined, error: unknown, isLoading: boolean } {
    return useQuery<Project[]>({
        queryKey: ['projects', 'favorites'],
        queryFn: async () => {
            let projects = await invoke_tauri_command('load_favorite_projects_command', {});
            return projects;
        }
    });
}
