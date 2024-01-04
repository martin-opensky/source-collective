-- This file should undo anything in `up.sql`
DROP INDEX story_chapters_story_id_index;
DROP TABLE story_chapters;
DROP INDEX story_media_media_id_index;
DROP INDEX story_media_story_id_index;
DROP TABLE story_media;
DROP INDEX story_quotes_quote_id_index;
DROP INDEX story_quotes_story_id_index;
DROP TABLE story_quotes;
DROP TABLE story;
