CREATE TABLE notes (
    id TEXT PRIMARY KEY NOT NULL,
    source_id TEXT NOT NULL,
    note TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (source_id) REFERENCES sources(id)
);

CREATE TABLE note_tags (
    id TEXT PRIMARY KEY NOT NULL,
    note_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (note_id) REFERENCES notes(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);

CREATE INDEX note_tags_note_id_index ON note_tags(note_id);
CREATE INDEX note_tags_tag_id_index ON note_tags(tag_id);