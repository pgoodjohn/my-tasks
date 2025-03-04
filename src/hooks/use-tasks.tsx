import { useQuery } from '@tanstack/react-query';
import type { Task } from '@/types';
import { invoke_tauri_command } from '@/lib/utils';

export function useTasks(showCompleted: boolean): { data: Array<Task> | undefined, error: unknown, isLoading: boolean, isError: boolean } {
    return useQuery<Array<Task>>({
        queryKey: ['tasks'],
        queryFn: async () => {
            const tasks = await invoke_tauri_command('load_tasks_command', { includeCompleted: showCompleted });
            return tasks;
        }
    });
}
