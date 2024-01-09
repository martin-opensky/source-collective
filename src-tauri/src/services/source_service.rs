extern crate uuid;
use diesel::prelude::*;
use crate::models::source_models::Source;
use crate::schema::sources::dsl;
use crate::schema::sources;
use crate::db::establish_db_connection;
use chrono::NaiveDateTime;

pub fn get_sources() -> Vec<Source> {
    let mut connection = establish_db_connection();
    dsl::sources.load::<Source>(&mut connection).expect("Error loading sources")
}

pub fn get_source_by_id(id: String) -> Source {
    let mut connection = establish_db_connection();
    
    dsl::sources
        .find(id)
        .first::<Source>(&mut connection)
        .expect("Error loading source")
}

pub fn create_source(name: String, image: Option<String>, source_type_id: i32, description: Option<String>, published_at: NaiveDateTime, primary_media_id: Option<String>) -> Source {
    let mut connection = establish_db_connection();
    
    let source_id = uuid::Uuid::new_v4().to_string();
    let new_source = Source {
        id: source_id.clone(),
        name,
        image,
        source_type_id,
        description,
        published_at,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
        primary_media_id,
    };

    diesel::insert_into(sources::table)
        .values(&new_source)
        .execute(&mut connection)
        .expect("Error saving new source");

    dsl::sources
        .find(source_id)
        .first::<Source>(&mut connection)
        .expect("Error loading source")
}

pub fn delete_source(id: String) -> Source {
    let mut connection = establish_db_connection();
    let source = dsl::sources
        .find(id.clone())
        .first::<Source>(&mut connection)
        .expect("Error loading source");

    diesel::delete(dsl::sources.find(id))
        .execute(&mut connection)
        .expect("Error deleting source");

    source
}