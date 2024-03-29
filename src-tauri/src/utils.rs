pub fn expand_home_dir(path: &str) -> String {
    if let Some(home_dir) = dirs::home_dir() {
        if path.starts_with("~/") {
            let suffix = &path.strip_prefix("~/").unwrap();
            let home_dir_path = home_dir.to_string_lossy();
            return format!("{}/{}", home_dir_path, suffix);
        }
    }
    path.to_owned()
}
