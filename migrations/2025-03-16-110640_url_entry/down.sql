-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS urls_update_at_trigger;

DROP TABLE urls;
