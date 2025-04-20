import { createFileRoute } from '@tanstack/react-router'
import { MarkdownTasksPage } from '../features/markdown-tasks'; // Assuming it's in the same directory

export const Route = createFileRoute('/markdown')({
    component: MarkdownTasksPage, // Use the imported component directly
}) 