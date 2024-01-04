CREATE TABLE story (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    image TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE story_quotes (
    id TEXT PRIMARY KEY NOT NULL,
    story_id TEXT NOT NULL,
    quote_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (story_id) REFERENCES story(id),
    FOREIGN KEY (quote_id) REFERENCES quotes(id)
);

CREATE INDEX story_quotes_story_id_index ON story_quotes(story_id);
CREATE INDEX story_quotes_quote_id_index ON story_quotes(quote_id);

CREATE TABLE story_media (
    id TEXT PRIMARY KEY NOT NULL,
    story_id TEXT NOT NULL,
    media_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (story_id) REFERENCES story(id),
    FOREIGN KEY (media_id) REFERENCES quotes(id)
);

CREATE INDEX story_media_story_id_index ON story_media(story_id);
CREATE INDEX story_media_media_id_index ON story_media(media_id);

CREATE TABLE story_chapters (
    id TEXT PRIMARY KEY NOT NULL,
    story_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,

    FOREIGN KEY (story_id) REFERENCES story(id)
);

CREATE INDEX story_chapters_story_id_index ON story_chapters(story_id);