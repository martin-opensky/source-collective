pub mod db;
pub mod models;
pub mod schema;
pub mod services;
pub mod commands;

use commands::source_commands::*;
use commands::greet_commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = fix_path_env::fix();

    tauri::Builder::default()
    .setup(|_app | {
        match db::init() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to initialize database: {}", e);
                Err(e)
            }
        }
    })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_sources,
            get_source_by_id,
            create_source,
            delete_source,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
