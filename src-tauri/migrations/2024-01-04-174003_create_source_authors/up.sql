CREATE TABLE authors (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    image TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE source_authors (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id),
    FOREIGN KEY (author_id) REFERENCES authors(id)
);

CREATE INDEX source_authors_source_id_index ON source_authors(source_id);
CREATE INDEX source_authors_author_id_index ON source_authors(author_id);