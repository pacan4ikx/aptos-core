-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS objects;
DROP INDEX IF EXISTS o_owner_idx;
DROP INDEX IF EXISTS o_object_skh_idx;
DROP INDEX IF EXISTS o_skh_idx;
DROP INDEX IF EXISTS o_insat_idx;
DROP TABLE IF EXISTS current_objects;
DROP INDEX IF EXISTS co_owner_idx;
DROP INDEX IF EXISTS co_object_skh_idx;
DROP INDEX IF EXISTS co_skh_idx;
DROP INDEX IF EXISTS co_insat_idx;
ALTER TABLE move_resources DROP COLUMN IF EXISTS state_key_hash;
DROP INDEX IF EXISTS mr_skh_idx;