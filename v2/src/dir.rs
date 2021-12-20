use std::fs::DirEntry;

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn has_git_subdir(entry: &DirEntry) -> bool {
    let suspect = entry.path().join(".git");
    suspect.exists() && suspect.is_dir()
}
