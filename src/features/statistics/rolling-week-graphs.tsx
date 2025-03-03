import { Bar, BarChart, CartesianGrid, XAxis } from "recharts"
import { useQuery } from '@tanstack/react-query';
import { useState } from "react";
import { endOfMonth, endOfWeek, startOfMonth, startOfWeek, subMonths, subWeeks } from "date-fns";
import type { DateRange } from "react-day-picker";
import type {
    ChartConfig} from "@/components/ui/chart";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart"
import { invoke_tauri_command } from "@/lib/utils";
import { Spinner } from "@/components/spinner";
import { DateRangePicker } from "@/components/date-range-picker";
import { Button } from "@/components/ui/button";

interface ProjectActivityStats {
    project_id: string;
    project_title: string;
    completed_tasks: number;
    created_tasks: number;
}

export function RollingWeekGraphs() {
    const [dateRange, setDateRange] = useState<DateRange | undefined>({
        from: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
        to: new Date(),
    });

    const setDateRangeForPeriod = (period: 'last-7-days' | 'last-14-days' | 'last-30-days' | 'this-week' | 'last-week' | 'this-month' | 'last-month') => {
        const now = new Date();
        switch (period) {
            case 'last-7-days':
                setDateRange({
                    from: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
                    to: now,
                });
                break;
            case 'last-14-days':
                setDateRange({
                    from: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000),
                    to: now,
                });
                break;
            case 'last-30-days':
                setDateRange({
                    from: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000),
                    to: now,
                });
                break;
            case 'this-week':
                setDateRange({
                    from: startOfWeek(now),
                    to: endOfWeek(now),
                });
                break;
            case 'last-week':
                const lastWeekStart = startOfWeek(subWeeks(now, 1));
                setDateRange({
                    from: lastWeekStart,
                    to: endOfWeek(lastWeekStart),
                });
                break;
            case 'this-month':
                setDateRange({
                    from: startOfMonth(now),
                    to: endOfMonth(now),
                });
                break;
            case 'last-month':
                const lastMonthStart = startOfMonth(subMonths(now, 1));
                setDateRange({
                    from: lastMonthStart,
                    to: endOfMonth(lastMonthStart),
                });
                break;
        }
    };

    const isActivePeriod = (period: 'last-7-days' | 'last-14-days' | 'last-30-days' | 'this-week' | 'last-week' | 'this-month' | 'last-month') => {
        if (!dateRange?.from || !dateRange.to) return false;

        const now = new Date();
        switch (period) {
            case 'last-7-days':
                const last7Days = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);
                return dateRange.from.getTime() === last7Days.getTime() &&
                    dateRange.to.getTime() === now.getTime();
            case 'last-14-days':
                const last14Days = new Date(Date.now() - 14 * 24 * 60 * 60 * 1000);
                return dateRange.from.getTime() === last14Days.getTime() &&
                    dateRange.to.getTime() === now.getTime();
            case 'last-30-days':
                const last30Days = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);
                return dateRange.from.getTime() === last30Days.getTime() &&
                    dateRange.to.getTime() === now.getTime();
            case 'this-week':
                return dateRange.from.getTime() === startOfWeek(now).getTime() &&
                    dateRange.to.getTime() === endOfWeek(now).getTime();
            case 'last-week':
                const lastWeekStart = startOfWeek(subWeeks(now, 1));
                return dateRange.from.getTime() === lastWeekStart.getTime() &&
                    dateRange.to.getTime() === endOfWeek(lastWeekStart).getTime();
            case 'this-month':
                return dateRange.from.getTime() === startOfMonth(now).getTime() &&
                    dateRange.to.getTime() === endOfMonth(now).getTime();
            case 'last-month':
                const lastMonthStart = startOfMonth(subMonths(now, 1));
                return dateRange.from.getTime() === lastMonthStart.getTime() &&
                    dateRange.to.getTime() === endOfMonth(lastMonthStart).getTime();
        }
    };

    return (
        <div className="flex flex-col gap-4">
            <div className="flex justify-between items-center mb-4">
                <div className="flex gap-2">
                    <Button
                        variant={isActivePeriod('last-7-days') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('last-7-days')}
                    >
                        Last 7 Days
                    </Button>
                    <Button
                        variant={isActivePeriod('last-14-days') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('last-14-days')}
                    >
                        Last 14 Days
                    </Button>
                    <Button
                        variant={isActivePeriod('last-30-days') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('last-30-days')}
                    >
                        Last 30 Days
                    </Button>
                    <Button
                        variant={isActivePeriod('this-week') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('this-week')}
                    >
                        This Week
                    </Button>
                    <Button
                        variant={isActivePeriod('last-week') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('last-week')}
                    >
                        Last Week
                    </Button>
                    <Button
                        variant={isActivePeriod('this-month') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('this-month')}
                    >
                        This Month
                    </Button>
                    <Button
                        variant={isActivePeriod('last-month') ? "default" : "outline"}
                        size="sm"
                        onClick={() => setDateRangeForPeriod('last-month')}
                    >
                        Last Month
                    </Button>
                </div>
                <DateRangePicker value={dateRange} onChange={setDateRange} />
            </div>
            <div className="flex justify-around">
                <div className="container max-w-[500px]">
                    <CompletedChart dateRange={dateRange} />
                </div>
                <div className="container max-w-[500px]">
                    <CreatedChart dateRange={dateRange} />
                </div>
            </div>
            <div className="flex justify-around">
                <div className="container max-w-[500px]">
                    <ProjectCompletedChart dateRange={dateRange} />
                </div>
                <div className="container max-w-[500px]">
                    <ProjectCreatedChart dateRange={dateRange} />
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

function useChartsData(dateRange: DateRange | undefined) {
    return useQuery({
        queryKey: ['tasks', 'completed', 'chart', dateRange],
        queryFn: async () => {
            if (!dateRange?.from || !dateRange.to) {
                throw new Error("Date range is required");
            }
            return invoke_tauri_command('load_rolling_week_day_charts_command', {
                since: dateRange.from.toISOString(),
                until: dateRange.to.toISOString(),
            })
        },
        enabled: !!dateRange?.from && !!dateRange.to,
    })
}

function CompletedChart({ dateRange }: { dateRange: DateRange | undefined }) {
    const chartsData = useChartsData(dateRange)

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
                <CardDescription>Selected date range</CardDescription>
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

function CreatedChart({ dateRange }: { dateRange: DateRange | undefined }) {
    const chartsData = useChartsData(dateRange)

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
                <CardDescription>Selected date range</CardDescription>
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

function ProjectCompletedChart({ dateRange }: { dateRange: DateRange | undefined }) {
    const chartsData = useQuery<Array<ProjectActivityStats>>({
        queryKey: ['tasks', 'project', 'activity', dateRange],
        queryFn: async () => {
            if (!dateRange?.from || !dateRange.to) {
                throw new Error("Date range is required");
            }
            return invoke_tauri_command('load_project_activity_stats_command', {
                since: dateRange.from.toISOString(),
                until: dateRange.to.toISOString(),
            })
        },
        enabled: !!dateRange?.from && !!dateRange.to,
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
                <CardDescription>Selected date range</CardDescription>
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
                            No completed tasks in the selected date range
                        </div>
                    )}
                </ChartContainer>
            </CardContent>
        </Card>
    )
}

function ProjectCreatedChart({ dateRange }: { dateRange: DateRange | undefined }) {
    const chartsData = useQuery<Array<ProjectActivityStats>>({
        queryKey: ['tasks', 'project', 'activity', dateRange],
        queryFn: async () => {
            if (!dateRange?.from || !dateRange.to) {
                throw new Error("Date range is required");
            }
            return invoke_tauri_command('load_project_activity_stats_command', {
                since: dateRange.from.toISOString(),
                until: dateRange.to.toISOString(),
            })
        },
        enabled: !!dateRange?.from && !!dateRange.to,
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
                <CardDescription>Selected date range</CardDescription>
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
                            No created tasks in the selected date range
                        </div>
                    )}
                </ChartContainer>
            </CardContent>
        </Card>
    )
}