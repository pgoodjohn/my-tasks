-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    project_id TEXT,
    due_at_utc DATETIME,
    deadline_at_utc DATETIME,
    created_at_utc DATETIME NOT NULL,
    completed_at_utc DATETIME,
    updated_at_utc DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    emoji TEXT,
    color TEXT,
    description TEXT,
    created_at_utc DATETIME NOT NULL,
    updated_at_utc DATETIME NOT NULL,
    archived_at_utc DATETIME
);