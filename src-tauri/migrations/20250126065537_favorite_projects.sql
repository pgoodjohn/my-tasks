-- Add migration script here
ALTER TABLE projects ADD COLUMN is_favorite BOOLEAN DEFAULT false;