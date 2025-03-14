-- Add migration script here
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    github_url TEXT
);
