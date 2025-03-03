import { useQuery } from '@tanstack/react-query';
import type { Task } from '@/types';
import { invoke_tauri_command } from '@/lib/utils';

export function useTasksDueToday(): { data: Array<Task> | undefined, error: unknown, isLoading: boolean, isError: boolean } {
    return useQuery<Array<Task>>({
        queryKey: ['tasks', 'due-today'],
        queryFn: async () => {
            const tasks = await invoke_tauri_command('load_tasks_due_today_command', { filter: 'overdue' });
            return tasks;
        }
    });
} 