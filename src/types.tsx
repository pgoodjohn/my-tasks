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