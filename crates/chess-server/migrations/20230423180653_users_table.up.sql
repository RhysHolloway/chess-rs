-- Add up migration script here
-- User table
CREATE TABLE IF NOT EXISTS users(
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    is_superuser BOOLEAN DEFAULT FALSE,
);
-- CREATE INDEX IF NOT EXISTS users_id_email_is_active_indx ON users (id, email, is_active);