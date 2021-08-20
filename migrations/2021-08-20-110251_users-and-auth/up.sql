CREATE TABLE users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    username varchar(32) UNIQUE NOT NULL,
    -- Hashed + salted representation of the username
    password text NOT NULL,
    -- Wether the user is currently blocked
    blocked boolean NOT NULL DEFAULT false,
    -- Wether the user is an admin
    admin boolean NOT NULL DEFAULT false
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
