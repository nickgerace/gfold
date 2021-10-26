use crate::dir;
use anyhow::Result;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub enum Status {
    Bare,
    Clean,
    Unclean,
    Unpushed,
}

#[derive(Clone)]
pub struct Report {
    pub path: String,
    pub parent: String,
    pub status: Status,
    pub status_as_string: String,
    pub branch: String,
    pub url: String,
}

pub struct Targets(pub Vec<PathBuf>);

impl Targets {
    pub fn generate_targets(&mut self, path: PathBuf) -> Result<()> {
        let entries = match fs::read_dir(&path) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("{}: {}", e, &path.display());
                return Ok(());
            }
        };
        for wrapped_entry in entries {
            let entry = wrapped_entry?;
            if entry.file_type()?.is_dir() && !dir::is_hidden(&entry) {
                match dir::has_git_subdir(&entry) {
                    true => self.0.push(entry.path()),
                    false => {
                        self.generate_targets(entry.path())?;
                    }
                }
            }
        }
        Ok(())
    }
}
