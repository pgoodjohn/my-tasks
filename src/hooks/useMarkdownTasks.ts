import { useQuery } from '@tanstack/react-query';
import { invoke_tauri_command } from '@/lib/utils';
import { MarkdownTask } from '@/types';

export function useMarkdownTasks() {
    return useQuery<MarkdownTask[], Error>({
        queryKey: ['markdownTasks'], // Unique query key
        queryFn: async () => {
            // invoke_tauri_command parses the JSON internally
            // Remove the type argument here as the function is not generic
            const result = await invoke_tauri_command('load_markdown_tasks_command', {});
            // We might need an explicit type assertion if TypeScript can't infer it
            return result as MarkdownTask[];
        },
        // Optional: Add configuration like staleTime, cacheTime, refetchOnWindowFocus etc.
        // staleTime: 5 * 60 * 1000, // 5 minutes
    });
} 