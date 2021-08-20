-- Users
CREATE TABLE users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    username varchar(32) UNIQUE NOT NULL,
    password text NOT NULL
);

-- Permissions that a user can have
CREATE TABLE permissions (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    user_id uuid REFERENCES users (id) NOT NULL,
    name varchar NOT NULL,

    UNIQUE (user_id, name)
);
