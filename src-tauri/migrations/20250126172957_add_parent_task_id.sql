-- Add migration script here
ALTER TABLE tasks ADD COLUMN parent_task_id TEXT;