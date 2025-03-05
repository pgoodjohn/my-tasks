import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import ReactMarkdown from "react-markdown";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { invoke_tauri_command } from "@/lib/utils";
import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
} from "@/components/ui/accordion";

export const Route = createFileRoute('/ollama')({
    component: OllamaPage
});

function OllamaPage() {
    const { data: prioritizationData, isLoading: isPrioritizationLoading, error: prioritizationError, refetch: refetchPrioritization } = useQuery({
        queryKey: ['tasks', 'prioritization'],
        queryFn: async () => {
            return await invoke_tauri_command("get_tasks_prioritization", {});
        },
        enabled: false,
    });

    const { data: quickTaskData, isLoading: isQuickTaskLoading, error: quickTaskError, refetch: refetchQuickTask } = useQuery({
        queryKey: ['tasks', 'quick'],
        queryFn: async () => {
            return await invoke_tauri_command("get_quick_task", {});
        },
        enabled: false,
    });

    return (
        <div className="space-y-4">
            <div className="flex flex-col gap-4">
                <h1 className="text-2xl font-bold">AI Task Assistant</h1>
                <p className="text-muted-foreground">
                    Use Ollama to help prioritize and organize your tasks. <br />This feature requires an Ollama instance running locally with the model specified in your settings.
                </p>
                <p className="text-muted-foreground">
                    <a href="https://ollama.com/" target="_blank" rel="noopener noreferrer">
                        Learn more about Ollama
                    </a>
                </p>
            </div>

            <div className="space-y-4">
                <div className="flex flex-col md:flex-row gap-4">
                    <Button
                        onClick={() => refetchPrioritization()}
                        disabled={isPrioritizationLoading || isQuickTaskLoading}
                        className="w-full md:w-auto"
                    >
                        {isPrioritizationLoading ? "Analyzing tasks..." : "Analyze All Tasks"}
                    </Button>

                    <Button
                        onClick={() => refetchQuickTask()}
                        disabled={isPrioritizationLoading || isQuickTaskLoading}
                        variant="secondary"
                        className="w-full md:w-auto"
                    >
                        {isQuickTaskLoading ? "Finding task..." : "Find Quick Task (30min)"}
                    </Button>
                </div>

                {(prioritizationError || quickTaskError) && (
                    <Card className="p-4 bg-destructive/10">
                        <h2 className="text-xl font-semibold mb-2 text-destructive">Error</h2>
                        <p className="text-sm text-destructive">Error analyzing tasks. Please ensure Ollama is running locally.</p>
                    </Card>
                )}

                {quickTaskData && (
                    <Card className="p-4">
                        <h2 className="text-xl font-semibold mb-2">Quick Task Suggestion</h2>
                        <p className="text-sm text-muted-foreground mb-4">Using model: {quickTaskData.model}</p>
                        {quickTaskData.thinking && (
                            <Accordion type="single" collapsible className="mb-4">
                                <AccordionItem value="thinking">
                                    <AccordionTrigger>View Thinking Process</AccordionTrigger>
                                    <AccordionContent>
                                        <div className="prose prose-sm dark:prose-invert text-muted-foreground max-w-none">
                                            <ReactMarkdown>{quickTaskData.thinking}</ReactMarkdown>
                                        </div>
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        )}
                        <div className="prose dark:prose-invert max-w-none">
                            <ReactMarkdown>{quickTaskData.response}</ReactMarkdown>
                        </div>
                    </Card>
                )}

                {prioritizationData && (
                    <Card className="p-4">
                        <h2 className="text-xl font-semibold mb-2">Task Analysis</h2>
                        <p className="text-sm text-muted-foreground mb-4">Using model: {prioritizationData.model}</p>
                        {prioritizationData.thinking && (
                            <Accordion type="single" collapsible className="mb-4">
                                <AccordionItem value="thinking">
                                    <AccordionTrigger>View Thinking Process</AccordionTrigger>
                                    <AccordionContent>
                                        <div className="prose prose-sm dark:prose-invert text-muted-foreground max-w-none">
                                            <ReactMarkdown>{prioritizationData.thinking}</ReactMarkdown>
                                        </div>
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        )}
                        <div className="prose dark:prose-invert max-w-none">
                            <ReactMarkdown>{prioritizationData.response}</ReactMarkdown>
                        </div>
                    </Card>
                )}
            </div>
        </div>
    );
} 