type Task = {
    id: string
    title: string
    description: string | null
    project: { id: string, title: string } | null
    due_at_utc: string | null
    created_at_utc: string
    updated_at_utc: string
    completed_at_utc: string | null
}

export type { Task }