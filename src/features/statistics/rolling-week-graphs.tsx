import { Bar, BarChart, CartesianGrid, XAxis } from "recharts"
import { useQuery } from '@tanstack/react-query';
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart"
import { invoke_tauri_command } from "@/lib/utils";
import { Spinner } from "@/components/spinner";

interface ProjectActivityStats {
    project_id: string;
    project_title: string;
    completed_tasks: number;
    created_tasks: number;
}

export function RollingWeekGraphs() {
    return (
        <div className="flex flex-col gap-4">
            <div className="flex justify-around">
                <div className="container max-w-[500px]">
                    <CompletedChart />
                </div>
                <div className="container max-w-[500px]">
                    <CreatedChart />
                </div>
            </div>
            <div className="flex justify-around">
                <div className="container max-w-[500px]">
                    <ProjectCompletedChart />
                </div>
                <div className="container max-w-[500px]">
                    <ProjectCreatedChart />
                </div>
            </div>
        </div>
    )
}

const chartConfig = {
    completed_tasks: {
        label: "Completed",
        color: "hsl(var(--chart-1))",
    },
    created_tasks: {
        label: "Created",
        color: "hsl(var(--chart-2))",
    },
    project_completed: {
        label: "Completed",
        color: "hsl(var(--chart-1))",
    },
    project_created: {
        label: "Created",
        color: "hsl(var(--chart-2))",
    }
} satisfies ChartConfig

function useChartsData() {
    return useQuery({
        queryKey: ['tasks', 'completed', 'chart'],
        queryFn: async () => {
            return invoke_tauri_command('load_rolling_week_day_charts_command', {})
        },
    })

}

function CompletedChart() {

    const chartsData = useChartsData()

    if (chartsData.isLoading) {
        return <><Spinner /></>
    }

    if (chartsData.isError) {
        return <div>An error occurred while fetching charts data.</div>
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle>Completed Tasks</CardTitle>
                <CardDescription>Last 7 days</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    {chartsData.data && (
                        <BarChart accessibilityLayer data={chartsData.data}>
                            <CartesianGrid vertical={false} />
                            <XAxis
                                dataKey="day"
                                tickLine={false}
                                tickMargin={10}
                                axisLine={false}
                                tickFormatter={(value) => value.slice(0, 3)}
                            />
                            <ChartTooltip
                                cursor={false}
                                content={<ChartTooltipContent hideLabel />}
                            />
                            <Bar dataKey="completed_tasks" fill="var(--color-completed_tasks)" radius={4} />
                        </BarChart>
                    )}
                </ChartContainer>
            </CardContent>
        </Card >
    )
}

function CreatedChart() {
    const chartsData = useChartsData()

    if (chartsData.isLoading) {
        return <><Spinner /></>
    }

    if (chartsData.isError) {
        return <div>An error occurred while fetching charts data.</div>
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle>Created Tasks</CardTitle>
                <CardDescription>Last 7 days</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    {chartsData.data && (
                        <BarChart accessibilityLayer data={chartsData.data}>
                            <CartesianGrid vertical={false} />
                            <XAxis
                                dataKey="day"
                                tickLine={false}
                                tickMargin={10}
                                axisLine={false}
                                tickFormatter={(value) => value.slice(0, 3)}
                            />
                            <ChartTooltip
                                cursor={false}
                                content={<ChartTooltipContent hideLabel />}
                            />
                            <Bar dataKey="created_tasks" fill="var(--color-created_tasks)" radius={4} />
                        </BarChart>
                    )}
                </ChartContainer>
            </CardContent>
        </Card >
    )
}

function ProjectCompletedChart() {
    const chartsData = useQuery<ProjectActivityStats[]>({
        queryKey: ['tasks', 'project', 'activity'],
        queryFn: async () => {
            return invoke_tauri_command('load_project_activity_stats_command', {})
        },
    })

    if (chartsData.isLoading) {
        return <><Spinner /></>
    }

    if (chartsData.isError) {
        return <div>An error occurred while fetching project activity data.</div>
    }

    const filteredData = chartsData.data?.filter((project: ProjectActivityStats) => project.completed_tasks > 0) || [];

    return (
        <Card>
            <CardHeader>
                <CardTitle>Completed Tasks by Project</CardTitle>
                <CardDescription>Last 7 days</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    {filteredData.length > 0 ? (
                        <BarChart accessibilityLayer data={filteredData}>
                            <CartesianGrid vertical={false} />
                            <XAxis
                                dataKey="project_title"
                                tickLine={false}
                                tickMargin={10}
                                axisLine={false}
                                angle={-45}
                                textAnchor="end"
                                height={100}
                            />
                            <ChartTooltip
                                cursor={false}
                                content={<ChartTooltipContent hideLabel />}
                            />
                            <Bar dataKey="completed_tasks" fill="var(--color-project_completed)" radius={4} />
                        </BarChart>
                    ) : (
                        <div className="flex items-center justify-center h-[200px] text-muted-foreground">
                            No completed tasks in the last 7 days
                        </div>
                    )}
                </ChartContainer>
            </CardContent>
        </Card>
    )
}

function ProjectCreatedChart() {
    const chartsData = useQuery<ProjectActivityStats[]>({
        queryKey: ['tasks', 'project', 'activity'],
        queryFn: async () => {
            return invoke_tauri_command('load_project_activity_stats_command', {})
        },
    })

    if (chartsData.isLoading) {
        return <><Spinner /></>
    }

    if (chartsData.isError) {
        return <div>An error occurred while fetching project activity data.</div>
    }

    const filteredData = chartsData.data?.filter((project: ProjectActivityStats) => project.created_tasks > 0) || [];

    return (
        <Card>
            <CardHeader>
                <CardTitle>Created Tasks by Project</CardTitle>
                <CardDescription>Last 7 days</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    {filteredData.length > 0 ? (
                        <BarChart accessibilityLayer data={filteredData}>
                            <CartesianGrid vertical={false} />
                            <XAxis
                                dataKey="project_title"
                                tickLine={false}
                                tickMargin={10}
                                axisLine={false}
                                angle={-45}
                                textAnchor="end"
                                height={100}
                            />
                            <ChartTooltip
                                cursor={false}
                                content={<ChartTooltipContent hideLabel />}
                            />
                            <Bar dataKey="created_tasks" fill="var(--color-project_created)" radius={4} />
                        </BarChart>
                    ) : (
                        <div className="flex items-center justify-center h-[200px] text-muted-foreground">
                            No created tasks in the last 7 days
                        </div>
                    )}
                </ChartContainer>
            </CardContent>
        </Card>
    )
}