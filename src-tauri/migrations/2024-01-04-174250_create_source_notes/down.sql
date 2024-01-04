-- This file should undo anything in `up.sql`
DROP INDEX note_tags_tag_id_index;
DROP INDEX note_tags_note_id_index;
DROP TABLE note_tags;
DROP TABLE notes;
