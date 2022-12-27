-- Sqlite Database Migration
CREATE TABLE IF NOT EXISTS users (
    id              INTEGER PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    email           TEXT NOT NULL,
    password        TEXT NOT NULL,
    user_id         TEXT NOT NULL
);

-- Have an username field and an email field.
-- Send a verification email to the user when they try to register.