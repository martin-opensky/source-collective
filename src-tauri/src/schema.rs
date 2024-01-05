// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Text,
        name -> Text,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    note_tags (id) {
        id -> Text,
        note_id -> Text,
        tag_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notes (id) {
        id -> Text,
        source_id -> Text,
        note -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    publisher (id) {
        id -> Text,
        name -> Text,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    quote_tags (id) {
        id -> Text,
        quote_id -> Text,
        tag_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    quotes (id) {
        id -> Text,
        source_id -> Text,
        author_id -> Nullable<Text>,
        quote -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    source_authors (id) {
        id -> Text,
        source_id -> Text,
        author_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    source_media (id) {
        id -> Text,
        name -> Text,
        image -> Nullable<Text>,
        source_id -> Text,
        source_media_type_id -> Integer,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    source_media_types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    source_publishers (id) {
        id -> Text,
        source_id -> Text,
        publisher_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    source_tags (id) {
        id -> Text,
        source_id -> Text,
        tag_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    source_types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    source_urls (id) {
        id -> Text,
        source_id -> Text,
        url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sources (id) {
        id -> Text,
        image -> Nullable<Text>,
        name -> Text,
        source_type_id -> Integer,
        description -> Nullable<Text>,
        published_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        primary_media_id -> Nullable<Text>,
    }
}

diesel::table! {
    story (id) {
        id -> Text,
        name -> Text,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    story_chapters (id) {
        id -> Text,
        story_id -> Text,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    story_media (id) {
        id -> Text,
        story_id -> Text,
        media_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    story_quotes (id) {
        id -> Text,
        story_id -> Text,
        quote_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        color -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(note_tags -> notes (note_id));
diesel::joinable!(note_tags -> tags (tag_id));
diesel::joinable!(notes -> sources (source_id));
diesel::joinable!(quote_tags -> sources (quote_id));
diesel::joinable!(quote_tags -> tags (tag_id));
diesel::joinable!(quotes -> authors (author_id));
diesel::joinable!(quotes -> sources (source_id));
diesel::joinable!(source_authors -> authors (author_id));
diesel::joinable!(source_authors -> sources (source_id));
diesel::joinable!(source_media -> source_media_types (source_media_type_id));
diesel::joinable!(source_publishers -> publisher (publisher_id));
diesel::joinable!(source_publishers -> sources (source_id));
diesel::joinable!(source_tags -> sources (source_id));
diesel::joinable!(source_tags -> tags (tag_id));
diesel::joinable!(source_urls -> sources (source_id));
diesel::joinable!(sources -> source_types (source_type_id));
diesel::joinable!(story_chapters -> story (story_id));
diesel::joinable!(story_media -> quotes (media_id));
diesel::joinable!(story_media -> story (story_id));
diesel::joinable!(story_quotes -> quotes (quote_id));
diesel::joinable!(story_quotes -> story (story_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    note_tags,
    notes,
    publisher,
    quote_tags,
    quotes,
    source_authors,
    source_media,
    source_media_types,
    source_publishers,
    source_tags,
    source_types,
    source_urls,
    sources,
    story,
    story_chapters,
    story_media,
    story_quotes,
    tags,
);
