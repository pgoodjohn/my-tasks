import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';
import { Task } from '@/types';

export function useTasksDueToday(): { data: Task[] | undefined, error: unknown, isLoading: boolean, isError: boolean } {
    return useQuery<Task[]>({
        queryKey: ['tasks', 'due-today'],
        queryFn: async () => {
            let tasks = await invoke_tauri_command('load_tasks_due_today_command', { filter: 'overdue' });
            return tasks;
        }
    });
} 