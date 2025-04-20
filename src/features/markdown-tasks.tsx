import React from 'react';
import { useMarkdownTasks } from '@/hooks/useMarkdownTasks';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { MarkdownTaskStatus } from '@/types'; // Import the enum

export function MarkdownTasksPage() {
    const { data: tasks, isLoading, error } = useMarkdownTasks();

    if (isLoading) {
        return <div>Loading Markdown Tasks...</div>;
    }

    if (error) {
        return <div className="text-red-500">Error loading tasks: {error.message}</div>;
    }

    if (!tasks || tasks.length === 0) {
        return (
            <Card>
                <CardHeader>
                    <CardTitle>Markdown Tasks</CardTitle>
                </CardHeader>
                <CardContent>
                    <p>No Markdown tasks found. Make sure the folder path is configured correctly and contains tasks in the format `[ ] Task Title` or `[x] Task Title`.</p>
                </CardContent>
            </Card>
        );
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle>Markdown Tasks</CardTitle>
            </CardHeader>
            <CardContent>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Title</TableHead>
                            <TableHead>Status</TableHead>
                            <TableHead>File</TableHead>
                            <TableHead>Line</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {tasks.map((task, index) => (
                            <TableRow key={`${task.file_path}-${task.line_number}-${index}`}>
                                <TableCell>{task.title}</TableCell>
                                <TableCell>
                                    <Badge variant={task.status === MarkdownTaskStatus.Completed ? "secondary" : "default"}>
                                        {task.status}
                                    </Badge>
                                </TableCell>
                                <TableCell className="text-xs text-muted-foreground">{task.file_path}</TableCell>
                                <TableCell>{task.line_number}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </CardContent>
        </Card>
    );
} 