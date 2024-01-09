use std::fs;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !db_file_exists() {
        create_db_file()?;
    }

    run_migrations()?;
    Ok(())
}

pub fn establish_db_connection() -> SqliteConnection {
    let db_path = get_db_path().clone();

    SqliteConnection::establish(db_path.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

fn run_migrations() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut connection = establish_connection()?;
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

fn create_db_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir)?;
    }

    fs::File::create(db_path)?;
    Ok(())
}

fn establish_connection() -> Result<SqliteConnection, diesel::ConnectionError> {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqliteConnection::establish(&db_path)
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let db_path = "/.source-collective/database.sqlite";

    println!("Database path: {}", home_dir.to_str().unwrap().to_string() + &db_path);
    home_dir.to_str().unwrap().to_string() + &db_path
}
