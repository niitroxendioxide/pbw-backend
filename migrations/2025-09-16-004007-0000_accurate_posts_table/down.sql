-- Drop tables in reverse order to handle foreign key constraints
DROP TABLE IF EXISTS ratings;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;

-- Drop the extension
DROP EXTENSION IF EXISTS "pgcrypto";-- This file should undo anything in `up.sql`
