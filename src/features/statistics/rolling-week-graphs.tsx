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

export function RollingWeekGraphs() {
    return (
        <div className="flex justify-around">
            <div className="container max-w-[500px]">
                <CompletedChart />
            </div>
            <div className="container max-w-[500px]">
                <CreatedChart />
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