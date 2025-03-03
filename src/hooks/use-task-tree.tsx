import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';
import { Task } from '@/types';

export function useTaskTree(): { data: Task[] | undefined, error: unknown, isLoading: boolean, isError: boolean } {
    return useQuery<Task[]>({
        queryKey: ['tasks', 'tree'],
        queryFn: async () => {
            let tasks = await invoke_tauri_command('load_tasks_command', { includeCompleted: true });
            return tasks;
        }
    });
} 