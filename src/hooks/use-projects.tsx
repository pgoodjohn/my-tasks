import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';
import { Project } from '@/types';
export function useProjects(): { data: Project[] | undefined, error: unknown, isLoading: boolean } {
    return useQuery<Project[]>({
        queryKey: ['projects'],
        queryFn: async () => {
            let projects = await invoke_tauri_command('load_projects_command', { showArchivedProjects: false });
            return projects;
        }
    });
}
