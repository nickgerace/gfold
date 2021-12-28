use crate::config::DisplayMode;
use crate::consts::NONE;
use crate::error::Error;
use crate::status::Status;
use crate::target_gen::Targets;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "windows")]
const NEWLINE: &str = "\r\n";
#[cfg(not(target_os = "windows"))]
const NEWLINE: &str = "\n";

#[derive(Clone, Debug)]
pub struct Report {
    pub path: String,
    pub parent: String,
    pub status: Status,
    pub status_as_string: String,
    pub branch: String,
    pub url: String,

    // Optional field that's only used in DisplayMode::Standard.
    pub email: Option<String>,
}

pub struct Reports(pub BTreeMap<String, Vec<Report>>);

impl Reports {
    pub fn new(
        targets: Targets,
        display_mode: &DisplayMode,
        git_path: &Option<PathBuf>,
    ) -> Result<Reports> {
        let include_email = match display_mode {
            DisplayMode::Standard => true,
            DisplayMode::Classic => false,
        };
        let git_path = match git_path {
            Some(s) => s.canonicalize()?,
            None => Path::new("git").to_path_buf(),
        };

        let unprocessed = targets
            .0
            .into_par_iter()
            .map(|path| generate_report(&path, &git_path, include_email))
            .collect::<Vec<Result<Report>>>();

        // Use a BTreeMap over a HashMap for sorted keys.
        let mut processed: Reports = Reports(BTreeMap::new());
        for wrapped_report in unprocessed {
            match wrapped_report {
                Ok(o) => {
                    let report = o.clone();
                    if let Some(mut s) = processed
                        .0
                        .insert(report.clone().parent, vec![report.clone()])
                    {
                        s.push(report.clone());
                        processed.0.insert(report.parent, s);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(processed)
    }
}

fn generate_report(repo_path: &Path, git_path: &Path, include_email: bool) -> Result<Report> {
    let git_shim = GitShim {
        working_directory: repo_path.to_path_buf(),
        git_path: git_path.to_path_buf(),
    };
    let branch = git_shim.get_branch()?;
    let status = match git_shim.is_bare()? {
        true => Status::Bare,
        false => match git_shim.is_unclean()? {
            true => Status::Unclean,
            false => match git_shim.is_unpushed(&branch)? {
                true => Status::Unpushed,
                false => Status::Clean,
            },
        },
    };
    let status_as_string = format!("{:?}", &status).to_lowercase();

    Ok(Report {
        path: match repo_path.file_name() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => {
                    return Err(Error::FileNameStrConversionFailure(repo_path.to_path_buf()).into())
                }
            },
            None => return Err(Error::FileNameNotFound(repo_path.to_path_buf()).into()),
        },
        parent: match repo_path.parent() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => return Err(Error::PathToStrConversionFailure(s.to_path_buf()).into()),
            },
            None => NONE.to_string(),
        },
        status,
        status_as_string,
        branch,
        url: git_shim.get_url()?,
        email: match include_email {
            true => Some(git_shim.get_email()?),
            false => None,
        },
    })
}

struct GitShim {
    working_directory: PathBuf,
    git_path: PathBuf,
}

impl GitShim {
    pub fn is_bare(&self) -> Result<bool> {
        let bare_output = self.git(&["rev-parse", "--is-bare-repository"])?;
        match bare_output.strip_suffix(NEWLINE) {
            Some(s) => Ok(s != "false"),
            None => Err(Error::StripNewLineFromStringFailure(bare_output).into()),
        }
    }

    pub fn is_unclean(&self) -> Result<bool> {
        Ok(!self.git(&["status", "--porcelain"])?.is_empty())
    }

    pub fn is_unpushed(&self, branch: &str) -> Result<bool> {
        Ok(!self
            .git(&["log", &format!("origin/{}..HEAD", branch)])?
            .is_empty())
    }

    pub fn get_branch(&self) -> Result<String> {
        let branch = self.git(&["rev-parse", "--abbrev-ref", "HEAD"])?;
        match branch.strip_suffix(NEWLINE) {
            Some(s) => Ok(s.to_string()),
            None => Err(Error::StripNewLineFromStringFailure(branch).into()),
        }
    }

    pub fn get_email(&self) -> Result<String> {
        Ok(
            match self
                .git(&["config", "--get", "user.email"])?
                .strip_suffix(NEWLINE)
            {
                Some(s) => s.to_string(),
                None => NONE.to_string(),
            },
        )
    }

    pub fn get_url(&self) -> Result<String> {
        Ok(
            match self
                .git(&["config", "--get", "remote.origin.url"])?
                .strip_suffix(NEWLINE)
            {
                Some(s) => s.to_string(),
                None => NONE.to_string(),
            },
        )
    }

    fn git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new(&self.git_path)
            .args(args)
            .current_dir(&self.working_directory)
            .output()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}
