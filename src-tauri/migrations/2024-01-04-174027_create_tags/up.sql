CREATE TABLE tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    color TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE source_tags (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE INDEX source_tags_source_id_index ON source_tags(source_id);
CREATE INDEX source_tags_tag_id_index ON source_tags(tag_id);