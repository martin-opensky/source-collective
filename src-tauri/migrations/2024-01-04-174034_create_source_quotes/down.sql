-- This file should undo anything in `up.sql`
DROP INDEX quote_tags_tag_id_index;
DROP INDEX quote_tags_quote_id_index;
DROP TABLE quote_tags;
DROP INDEX quotes_author_id_index;
DROP INDEX quotes_source_id_index;
DROP TABLE quotes;
