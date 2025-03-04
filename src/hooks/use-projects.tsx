import { useQuery } from '@tanstack/react-query';
import type { Project } from '@/types';
import { invoke_tauri_command } from '@/lib/utils';

export function useProjects(): { data: Array<Project> | undefined, error: unknown, isLoading: boolean } {
    return useQuery<Array<Project>>({
        queryKey: ['projects'],
        queryFn: async () => {
            const projects = await invoke_tauri_command('load_projects_command', { showArchivedProjects: false });
            return projects;
        }
    });
}
