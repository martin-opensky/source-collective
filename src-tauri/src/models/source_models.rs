use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::sources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Source {
    pub id: String,
    pub image: Option<String>,
    pub name: String,
    pub source_type_id: i32,
    pub description: Option<String>,
    pub published_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub primary_media_id: Option<String>,
}