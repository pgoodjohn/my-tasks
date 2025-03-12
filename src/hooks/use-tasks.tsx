import { useQuery } from '@tanstack/react-query';
import type { Task } from '@/types';
import { invoke_tauri_command } from '@/lib/utils';

export function useTasks(includeCompleted: boolean): { data: Array<Task> | undefined, error: unknown, isLoading: boolean, isError: boolean } {
    return useQuery<Array<Task>>({
        queryKey: ['tasks', { includeCompleted }],
        queryFn: async () => {
            const tasks = await invoke_tauri_command('load_tasks_command', { includeCompleted: includeCompleted });
            return tasks;
        },
        staleTime: 0, // Consider data stale immediately
        refetchOnWindowFocus: true, // Refetch when window regains focus
        refetchOnMount: true, // Refetch when component mounts
        refetchOnReconnect: true, // Refetch when network reconnects
    });
}
