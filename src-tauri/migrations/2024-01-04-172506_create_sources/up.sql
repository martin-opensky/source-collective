CREATE TABLE source_types (
    id INT(11) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

INSERT INTO source_types (id, name) VALUES (1, 'Book');
INSERT INTO source_types (id, name) VALUES (2, 'Article');
INSERT INTO source_types (id, name) VALUES (3, 'Video');
INSERT INTO source_types (id, name) VALUES (4, 'Audio');
INSERT INTO source_types (id, name) VALUES (5, 'Other');


CREATE TABLE sources (
    id  TEXT PRIMARY KEY NOT NULL,
    image TEXT,
    name TEXT NOT NULL,
    source_type_id INT(11) NOT NULL,
    description TEXT,
    -- primary_media_id TEXT,
    published_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_type_id) REFERENCES source_types(id)
);

CREATE INDEX sources_source_type_id_index ON sources(source_type_id);

CREATE TABLE source_urls (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id)
);

CREATE INDEX source_urls_source_id_index ON source_urls(source_id);