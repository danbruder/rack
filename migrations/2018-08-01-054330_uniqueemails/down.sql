-- This file should undo anything in `up.sql`
ALTER TABLE users 
DROP INDEX users_email;
