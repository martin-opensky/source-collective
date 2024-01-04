CREATE TABLE quotes (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    author_id TEXT,
    quote TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id),
    FOREIGN KEY (author_id) REFERENCES authors(id)
);

CREATE INDEX quotes_source_id_index ON quotes(source_id);
CREATE INDEX quotes_author_id_index ON quotes(author_id);

CREATE TABLE quote_tags (
    id TEXT PRIMARY KEY NOT NULL,
    quote_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (quote_id) REFERENCES sources(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE INDEX quote_tags_quote_id_index ON quote_tags(quote_id);
CREATE INDEX quote_tags_tag_id_index ON quote_tags(tag_id);