-- Add migration script here
CREATE TABLE IF NOT EXISTS recurring_tasks (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    frequency TEXT NOT NULL, -- 'daily', 'weekly', 'monthly', 'yearly'
    interval INTEGER NOT NULL DEFAULT 1, -- e.g., every 2 weeks
    next_due_at_utc DATETIME NOT NULL,
    created_at_utc DATETIME NOT NULL,
    updated_at_utc DATETIME NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
); 