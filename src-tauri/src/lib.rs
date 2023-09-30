#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use git2::Repository;
use serde::Serialize;

pub mod commands;
pub mod db;
pub mod events;
pub mod utils;

pub use commands::*;

#[derive(Debug, Serialize)]
pub struct RepoInfo {
    pub full_path: String,
    pub name: String,
    pub branch_name: String,
}

impl RepoInfo {
    #[must_use]
    pub fn from_repository(repo: &Repository) -> Self {
        let full_path = repo.path().parent().map_or_else(
            || String::from("No path found"),
            |path| path.to_string_lossy().to_string(),
        );
        let name = full_path.split('/').last().unwrap_or(&full_path);
        println!("full_path: {full_path:?}");
        println!("name: {name:?}");
        repo.head().map_or_else(
            |_| Self {
                full_path: full_path.to_string(),
                name: name.to_string(),
                branch_name: String::from("No branch found"),
            },
            |head| {
                let branch_name = head.shorthand().unwrap_or("Detached HEAD");
                Self {
                    full_path: full_path.to_string(),
                    name: name.to_string(),
                    branch_name: String::from(branch_name),
                }
            },
        )
    }
}
