use super::RepoInfo;
use crate::db::establish_db_connection;

use git2::Repository;
use sqlx::Row;
use std::path::Path;

/// # Errors
///
/// Will return `Err` if it fails to add the repo to the database.
#[tauri::command]
pub async fn remove_repo(repo_path: String) -> Result<bool, String> {
    let db = establish_db_connection().await;

    match sqlx::query("DELETE FROM repos WHERE path = $1")
        .bind(repo_path)
        .execute(&db)
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Error adding repo to database: {e}")),
    }
}

/// # Errors
///
/// Will return `Err` if it fails to add the repo to the database.
#[tauri::command]
pub async fn add_new_repo(repo_path: String) -> Result<bool, String> {
    let db = establish_db_connection().await;

    match sqlx::query("INSERT INTO repos (path) VALUES ($1) ON CONFLICT DO NOTHING")
        .bind(repo_path)
        .execute(&db)
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Error adding repo to database: {e}")),
    }
}

#[must_use]
#[tauri::command]
pub async fn list_repos() -> Vec<RepoInfo> {
    let db = establish_db_connection().await;
    // Fetch all paths from the database and turn them into RepoInfo structs.
    (sqlx::query("SELECT path FROM repos").fetch_all(&db).await).map_or_else(
        |_| Vec::new(),
        |repo_rows| {
            repo_rows
                .iter()
                .map(|row| {
                    let repo_path: String = row.get("path");
                    let path = Path::new(&repo_path);
                    let path_str = repo_path.split('/').last().unwrap_or(&repo_path).to_owned();
                    let full_path_str = path.to_string_lossy();

                    Repository::open(path).map_or_else(
                        |_| RepoInfo {
                            full_path: full_path_str.to_string(),
                            name: path_str.to_string(),
                            branch_name: String::from("No branch found"),
                        },
                        |repo| RepoInfo::from_repository(&repo),
                    )
                })
                .collect()
        },
    )
}
