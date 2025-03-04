import { Link, useRouterState  } from '@tanstack/react-router'
import React from "react";
import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import { useProjects } from "@/hooks/use-projects";
import { useTaskTree } from "@/hooks/use-task-tree";

export default function AppBreadcrumb() {
    return (
        <div className="overflow-x-auto">
            <Breadcrumb>
                <BreadcrumbList className="flex-nowrap">
                    <BreadcrumbItems />
                </BreadcrumbList>
            </Breadcrumb>
        </div>
    )
}

function BreadcrumbItems() {
    const routerState = useRouterState();
    const match = routerState.matches[1];

    if (match.routeId == "/") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Home
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    return (
        <>
            <BreadcrumbItem>
                <BreadcrumbLink asChild>
                    <Link to='/'>Home</Link>
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItemFromRouterMatch match={match} />
        </>
    )
}

function BreadcrumbItemFromRouterMatch({ match }: any) {

    if (match.routeId == "/tasks/") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Tasks
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    if (match.routeId == "/tasks/completed") {
        return (
            <>
                <BreadcrumbItem>
                    <BreadcrumbLink asChild>
                        <Link to="/tasks">Tasks</Link>
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                    <BreadcrumbPage>
                        Completed Tasks
                    </BreadcrumbPage>
                </BreadcrumbItem>
            </>
        )
    }

    if (match.routeId.startsWith("/tasks/")) {
        const taskId = match.id.match(/^\/tasks\/([^/]+)/)?.[1];
        return (
            <TaskBreadcrumb taskId={taskId} />
        )
    }

    if (match.routeId.startsWith("/projects")) {
        const projectId = match.id.match(/^\/projects\/([^/]+)/)?.[1];
        return (
            <ProjectBreadcrumb projectId={projectId} />
        )
    }

    if (match.routeId == "/statistics") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Statistics
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    if (match.routeId == "/settings") {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Settings
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    return (
        <>
        </>
    )
}

function ProjectBreadcrumb({ projectId }: { projectId: string | undefined }) {

    const projects = useProjects();

    if (!projectId) {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Projects
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    const project = projects.data?.find((item) => item.id === projectId);

    return (
        <>
            <BreadcrumbItem>
                <BreadcrumbLink asChild>
                    <Link to="/projects">Projects</Link>
                </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
                <BreadcrumbPage>
                    {project?.title}
                </BreadcrumbPage>
            </BreadcrumbItem >
        </>
    )
}

function TaskBreadcrumb({ taskId }: { taskId: string | undefined }) {
    const { data: tasks } = useTaskTree();

    if (!taskId) {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Tasks
                </BreadcrumbPage>
            </BreadcrumbItem>
        )
    }

    const task = tasks?.find(t => t.id === taskId);
    if (!task) {
        return (
            <BreadcrumbItem>
                <BreadcrumbPage>
                    Loading...
                </BreadcrumbPage>
            </BreadcrumbItem>
        );
    }

    // Build the task hierarchy chain
    const taskChain: Array<typeof task> = [];
    let currentTask = task;
    taskChain.push(currentTask);

    while (currentTask.parent_task_id) {
        const parentTask = tasks?.find(t => t.id === currentTask.parent_task_id);
        if (!parentTask) break;
        taskChain.push(parentTask);
        currentTask = parentTask;
    }

    // Reverse the chain so it goes from root to leaf
    taskChain.reverse();

    return (
        <>
            <BreadcrumbItem>
                <BreadcrumbLink asChild>
                    <Link to="/tasks">Tasks</Link>
                </BreadcrumbLink>
            </BreadcrumbItem>
            {taskChain.map((t, index) => (
                <React.Fragment key={t.id}>
                    <BreadcrumbSeparator />
                    <BreadcrumbItem>
                        {index === taskChain.length - 1 ? (
                            <BreadcrumbPage>
                                {t.title}
                            </BreadcrumbPage>
                        ) : (
                            <BreadcrumbLink asChild>
                                <Link to="/tasks/$taskId" params={{ taskId: t.id }}>
                                    {t.title}
                                </Link>
                            </BreadcrumbLink>
                        )}
                    </BreadcrumbItem>
                </React.Fragment>
            ))}
        </>
    )
}