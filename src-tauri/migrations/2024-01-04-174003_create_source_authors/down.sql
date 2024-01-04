-- This file should undo anything in `up.sql`
DROP INDEX source_authors_author_id_index;
DROP INDEX source_authors_source_id_index;
DROP TABLE source_authors;
DROP TABLE authors;
