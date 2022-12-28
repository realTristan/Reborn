-- Sqlite Database Migration
CREATE TABLE IF NOT EXISTS users (
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    email               TEXT NOT NULL,
    hwid                TEXT NOT NULL,
    unique_hwid_count   INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS tokens (
    id                  INTEGER PRIMARY KEY NOT NULL,       -- id auto increment
    token               TEXT NOT NULL,                      -- sha256 encoded token
    channel             INTEGER NOT NULL,                   -- channel id
    createdby           INTEGER NOT NULL,                   -- sha256(creator_user_id + extra_string)
    createdat           INTEGER NOT NULL                    -- when the token was created. If the token is older then 24 hours it will be deleted
);


-- HWID
--
-- The hwid is sha256 encrypted
-- Change the hwid field if the hwid is different then increase the unique_hwid_count by 1


-- Registration
--
-- Have an username field and an email field.
-- Send a verification email to the user when they try to register.
