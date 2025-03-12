declare module "@tanstack/react-router" {
    interface Register {
        search: { showCompleted: boolean };
    }
    interface FileRoutesByPath {
        "/": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/ollama": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/settings": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/tasks/": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/projects/": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/projects/$projectId": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/tasks/$taskId": {
            parentRoute: typeof import("./routes/__root").Route;
        };
        "/tasks/completed": {
            parentRoute: typeof import("./routes/__root").Route;
        };
    }
}

type Task = {
    id: string
    title: string
    description: string | null
    project_id: string | null
    parent_task_id: string | null
    due_at_utc: string | null
    created_at_utc: string
    updated_at_utc: string
    completed_at_utc: string | null
}

export type { Task }

type Project = {
    id: string
    title: string
    emoji: string | null
    color: string | null
    description: string | null
    isFavorite: boolean
}

export type { Project }

export enum Frequency {
    Daily = "daily",
    Weekly = "weekly",
    Monthly = "monthly",
    Yearly = "yearly"
}

type RecurringTask = {
    id: string
    task_id: string
    frequency: Frequency
    interval: number
    next_due_at_utc: string
    created_at_utc: string
    updated_at_utc: string
}

export type { RecurringTask }