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
    const { data, isLoading, error, refetch } = useQuery({
        queryKey: ['tasks', 'prioritization'],
        queryFn: async () => {
            return await invoke_tauri_command("get_tasks_prioritization", {});
        },
        enabled: false, // Don't fetch automatically on component mount
    });

    return (
        <div className="space-y-4">
            <div className="flex flex-col gap-4">
                <h1 className="text-2xl font-bold">AI Task Assistant</h1>
                <p className="text-muted-foreground">
                    Use Ollama to help prioritize and organize your tasks. This feature requires an Ollama instance running locally with the Mistral model.
                </p>
            </div>

            <div className="space-y-4">
                <Button
                    onClick={() => refetch()}
                    disabled={isLoading}
                    className="w-full md:w-auto"
                >
                    {isLoading ? "Analyzing tasks..." : "Analyze Tasks with Ollama"}
                </Button>

                {error && (
                    <Card className="p-4 bg-destructive/10">
                        <h2 className="text-xl font-semibold mb-2 text-destructive">Error</h2>
                        <p className="text-sm text-destructive">Error getting task prioritization. Please ensure Ollama is running locally.</p>
                    </Card>
                )}

                {data && (
                    <Card className="p-4">
                        <h2 className="text-xl font-semibold mb-2">Analysis Results</h2>
                        <p className="text-sm text-muted-foreground mb-4">Using model: {data.model}</p>
                        {data.thinking && (
                            <Accordion type="single" collapsible className="mb-4">
                                <AccordionItem value="thinking">
                                    <AccordionTrigger>View Thinking Process</AccordionTrigger>
                                    <AccordionContent>
                                        <div className="prose prose-sm dark:prose-invert text-muted-foreground max-w-none">
                                            <ReactMarkdown>{data.thinking}</ReactMarkdown>
                                        </div>
                                    </AccordionContent>
                                </AccordionItem>
                            </Accordion>
                        )}
                        <div className="prose dark:prose-invert max-w-none">
                            <ReactMarkdown>{data.response}</ReactMarkdown>
                        </div>
                    </Card>
                )}
            </div>
        </div>
    );
} 