-- Users
CREATE TABLE users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    username varchar(32) UNIQUE NOT NULL,
    -- Hashed + salted representation of the username
    password text NOT NULL,
    -- Wether the user is currently blocked
    blocked boolean NOT NULL DEFAULT false
);

-- Permissions that a user can have
CREATE TABLE permissions (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    user_id uuid REFERENCES users (id) NOT NULL,
    name varchar(64) NOT NULL,

    UNIQUE (user_id, name)
);

-- Security reports (e.g. when a user is blocked)
CREATE TABLE security_reports (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    -- When the report was made
    report_time timestamp NOT NULL DEFAULT now(),
    -- What type of report it is
    report_type varchar(64) NOT NULL,
    -- Contents of the report
    content TEXT NOT NULL
);

-- Stores refresh tokens
CREATE TABLE refresh_tokens (
    -- This is more efficient than storing the text
    token bytea PRIMARY KEY,
    -- The user for whom the token was created
    user_id uuid NOT NULL REFERENCES users(id),
    -- When the token expires
    expires_at timestamp NOT NULL,
    -- When the token was last used (is NULL until used)
    last_used_at timestamp
);
