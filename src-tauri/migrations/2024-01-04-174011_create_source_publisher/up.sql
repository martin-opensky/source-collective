CREATE TABLE publisher (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    image TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE source_publishers (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    publisher_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id),
    FOREIGN KEY (publisher_id) REFERENCES publisher(id)
);

CREATE INDEX source_publishers_source_id_index ON source_publishers(source_id);
CREATE INDEX source_publishers_publisher_id_index ON source_publishers(publisher_id);
