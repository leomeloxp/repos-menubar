#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use git2::Repository;
use serde::Serialize;

pub mod commands;
pub mod events;
pub mod utils;

pub use commands::*;

#[derive(Debug, Serialize)]
pub struct RepoInfo {
    pub full_path: String,
    pub path: String,
    pub branch_name: String,
}

impl RepoInfo {
    #[must_use]
    pub fn new(repo: &Repository, path_str: &str, full_path_str: &str) -> Self {
        repo.head().map_or_else(
            |_| Self {
                full_path: String::from(full_path_str),
                path: String::from(path_str),
                branch_name: String::from("No branch found"),
            },
            |head| {
                let branch_name = head.shorthand().unwrap_or("Detached HEAD");
                Self {
                    full_path: String::from(full_path_str),
                    path: String::from(path_str),
                    branch_name: String::from(branch_name),
                }
            },
        )
    }
}
