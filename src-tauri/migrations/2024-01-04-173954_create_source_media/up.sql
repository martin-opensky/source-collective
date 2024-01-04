CREATE TABLE source_media_types (
    id INT(11) PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

INSERT INTO source_media_types (id, name) VALUES (1, 'Image');
INSERT INTO source_media_types (id, name) VALUES (2, 'Video');
INSERT INTO source_media_types (id, name) VALUES (3, 'Audio');
INSERT INTO source_media_types (id, name) VALUES (4, 'Document');
INSERT INTO source_media_types (id, name) VALUES (5, 'Text');
INSERT INTO source_media_types (id, name) VALUES (6, 'Other');

CREATE TABLE source_media (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    image TEXT,
    source_id TEXT NOT NULL,
    source_media_type_id INT(11) NOT NULL,
    content TEXT NOT NULL, -- URL or file path
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id),
    FOREIGN KEY (source_media_type_id) REFERENCES source_media_types(id)
);

CREATE INDEX source_media_source_id_index ON source_media(source_id);

ALTER TABLE sources ADD COLUMN primary_media_id TEXT REFERENCES source_media(id);

CREATE INDEX sources_primary_media_id_index ON sources(primary_media_id);