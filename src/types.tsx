type Task = {
    id: string
    title: string
    description: string | null
    project: Project | null
    due_at_utc: string | null
    deadline_at_utc: string | null
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
}

export type { Project }