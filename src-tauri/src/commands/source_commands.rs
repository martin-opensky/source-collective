use crate::models::source_models::Source;
use chrono::NaiveDateTime;
use crate::services::source_service;

#[tauri::command]
pub fn get_sources() -> Vec<Source> {
    source_service::get_sources()
}

#[tauri::command]
pub fn get_source_by_id(id: String) -> Source {
    source_service::get_source_by_id(id)
}

#[tauri::command]
pub fn create_source(name: String, image: Option<String>, source_type_id: i32, description: Option<String>, published_at: NaiveDateTime, primary_media_id: Option<String>) -> Source {
    source_service::create_source(name, image, source_type_id, description, published_at, primary_media_id)
}

#[tauri::command]
pub fn delete_source(id: String) -> Source {
    source_service::delete_source(id)
}