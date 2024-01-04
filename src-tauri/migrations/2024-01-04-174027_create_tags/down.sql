-- This file should undo anything in `up.sql`
DROP INDEX source_tags_tag_id_index;
DROP INDEX source_tags_source_id_index;
DROP TABLE source_tags;
DROP TABLE tags;
