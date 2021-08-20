-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS users, permissions, refresh_tokens, security_reports CASCADE;
