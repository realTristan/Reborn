-- Sqlite Database Migration
CREATE TABLE IF NOT EXISTS users (
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    email               TEXT NOT NULL,
    password            TEXT NOT NULL,
    user_id             TEXT NOT NULL,
    hwid                TEXT NOT NULL,
    unique_hwid_count   INTEGER NOT NULL
);


-- HWID
--
-- The hwid is sha256 encrypted
-- Change the hwid field if the hwid is different then increase the unique_hwid_count by 1


-- Registration
--
-- Have an username field and an email field.
-- Send a verification email to the user when they try to register.
