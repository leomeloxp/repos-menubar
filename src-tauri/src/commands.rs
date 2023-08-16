use super::RepoInfo;
use crate::utils::expand_home_dir;

use git2::Repository;
use std::fs;

const IGNORED_FILES: [&str; 1] = [".DS_Store"];

#[tauri::command]
pub fn list_repos(repo_path: &str) -> Vec<RepoInfo> {
    let mut result = Vec::new();

    let expanded_path = expand_home_dir(repo_path);

    // Get a list of directories in the repository path
    let dirs = match fs::read_dir(&expanded_path) {
        Ok(dirs) => dirs,
        Err(e) => {
            println!("Failed to read directory: {}", e);
            return Vec::new();
        }
    };

    for dir in dirs.into_iter().flatten() {
        let path = dir.path();
        // Skip ignored files
        if let Some(entry_name) = path.file_name() {
            let name_str = entry_name.to_string_lossy();
            if IGNORED_FILES.contains(&name_str.as_ref()) {
                continue;
            }
        }

        let path_str = path
            .strip_prefix(&expanded_path)
            .unwrap_or(&path)
            .to_string_lossy();
        let full_path_str = path.to_string_lossy();

        if let Ok(repo) = Repository::open(dir.path()) {
            result.push(RepoInfo::new(repo, &path_str, &full_path_str));
        }
    }

    result
}
