use std::fs;
use std::path::Path;

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

// Check if a database file exists, and create one if it does not.
pub async fn init() {
    if !db_file_exists() {
        create_db_file();
    }

    let _ = run_migrations().await;
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

pub async fn establish_db_connection() -> SqlitePool {
    establish_connection().await
}

async fn run_migrations() {
    let pool = establish_connection().await;

    for query in &MIGRATIONS {
        sqlx::query(query).execute(&pool).await.unwrap();
    }

    // sqlx::query("INSERT INTO repos (path) VALUES (?)")
    //     .bind("/Users/leomeloxp/Development/TS/bingo-cards-lite")
    //     .execute(&pool)
    //     .await
    //     .unwrap();
}

async fn establish_connection() -> SqlitePool {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {db_path}"))
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
    dirs::home_dir().map_or_else(
        || panic!("Failed to get home directory"),
        |home_dir| {
            format!(
                "{}/.config/repos-menubar/database.sqlite",
                home_dir.to_str().unwrap_or("~")
            )
        },
    )
}

const MIGRATIONS: [&str; 1] = [r#"
    CREATE TABLE IF NOT EXISTS repos (
        path TEXT PRIMARY KEY
    );
    "#];
