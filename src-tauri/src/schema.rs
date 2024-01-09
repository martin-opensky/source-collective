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
    publisher (id) {
        id -> Text,
        name -> Text,
        image -> Nullable<Text>,
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

diesel::joinable!(source_authors -> authors (author_id));
diesel::joinable!(source_authors -> sources (source_id));
diesel::joinable!(source_media -> source_media_types (source_media_type_id));
diesel::joinable!(source_publishers -> publisher (publisher_id));
diesel::joinable!(source_publishers -> sources (source_id));
diesel::joinable!(source_urls -> sources (source_id));
diesel::joinable!(sources -> source_types (source_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    publisher,
    source_authors,
    source_media,
    source_media_types,
    source_publishers,
    source_types,
    source_urls,
    sources,
);
