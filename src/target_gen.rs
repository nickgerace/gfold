use anyhow::Result;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tracing::{error, warn};

pub struct Targets(pub Vec<PathBuf>);

impl Targets {
    pub fn new(path: &Path) -> Result<Targets> {
        let mut targets = Targets(Vec::new());
        targets.generate_targets(path)?;
        Ok(targets)
    }

    fn generate_targets(&mut self, path: &Path) -> Result<()> {
        let entries = match fs::read_dir(&path) {
            Ok(o) => o,
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                warn!("{}: {}", e, &path.display());
                return Ok(());
            }
            Err(e) => {
                error!("{}: {}", e, &path.display());
                return Ok(());
            }
        };
        for wrapped_entry in entries {
            let entry = wrapped_entry?;
            if entry.file_type()?.is_dir() && !is_hidden(&entry) {
                match has_git_subdir(&entry) {
                    true => self.0.push(entry.path()),
                    false => {
                        self.generate_targets(&entry.path())?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn has_git_subdir(entry: &DirEntry) -> bool {
    let suspect = entry.path().join(".git");
    suspect.exists() && suspect.is_dir()
}
