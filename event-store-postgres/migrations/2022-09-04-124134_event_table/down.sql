-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS event;
DROP EXTENSION IF EXISTS "uuid-ossp" CASCADE;
DROP EXTENSION IF EXISTS hstore CASCADE;
