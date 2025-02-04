-- Add migration script here
ALTER TABLE tasks ADD COLUMN ticks INT DEFAULT 0;