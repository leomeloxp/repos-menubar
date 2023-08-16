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
    pub fn new(repo: Repository, path_str: &str, full_path_str: &str) -> Self {
        if let Ok(head) = repo.head() {
            let branch_name = head.shorthand().unwrap_or("Detached HEAD");
            RepoInfo {
                full_path: String::from(full_path_str),
                path: String::from(path_str),
                branch_name: String::from(branch_name),
            }
        } else {
            RepoInfo {
                full_path: String::from(full_path_str),
                path: String::from(path_str),
                branch_name: String::from("No branch found"),
            }
        }
    }
}
