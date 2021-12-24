use crate::config::DisplayMode;
use crate::error::Error;
use crate::status::Status;
use crate::target_gen::Targets;
use anyhow::Result;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

pub const NONE: &str = "none";

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
    pub fn new(targets: Targets, display_mode: &DisplayMode) -> Result<Reports> {
        let include_email = match display_mode {
            DisplayMode::Standard => true,
            DisplayMode::Classic => false,
        };

        let unprocessed = targets
            .0
            .into_par_iter()
            .map(|m| generate_report(&m, include_email))
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

fn generate_report(path: &Path, include_email: bool) -> Result<Report> {
    let branch = git(&["rev-parse", "--abbrev-ref", "HEAD"], path)?;
    let branch = match branch.strip_suffix(NEWLINE) {
        Some(s) => s,
        None => return Err(Error::StripNewLineFromStringFailure(branch).into()),
    };
    let status = match is_bare(path)? {
        true => Status::Bare,
        false => match is_unclean(path)? {
            true => Status::Unclean,
            false => match is_unpushed(path, branch)? {
                true => Status::Unpushed,
                false => Status::Clean,
            },
        },
    };
    let status_as_string = format!("{:?}", &status).to_lowercase();

    Ok(Report {
        path: match path.file_name() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => return Err(Error::FileNameStrConversionFailure(path.to_path_buf()).into()),
            },
            None => return Err(Error::FileNameNotFound(path.to_path_buf()).into()),
        },
        parent: match path.parent() {
            Some(s) => match s.to_str() {
                Some(s) => s.to_string(),
                None => return Err(Error::PathToStrConversionFailure(s.to_path_buf()).into()),
            },
            None => NONE.to_string(),
        },
        status,
        status_as_string,
        branch: branch.to_string(),
        url: match git(&["config", "--get", "remote.origin.url"], path)?.strip_suffix(NEWLINE) {
            Some(s) => s.to_string(),
            None => NONE.to_string(),
        },
        email: match include_email {
            true => Some(get_email(path)?),
            false => None,
        },
    })
}

fn is_bare(path: &Path) -> Result<bool> {
    let bare_output = git(&["rev-parse", "--is-bare-repository"], path)?;
    match bare_output.strip_suffix(NEWLINE) {
        Some(s) => Ok(s != "false"),
        None => Err(Error::StripNewLineFromStringFailure(bare_output).into()),
    }
}

fn is_unclean(path: &Path) -> Result<bool> {
    Ok(!git(&["status", "--porcelain"], path)?.is_empty())
}

fn is_unpushed(path: &Path, branch: &str) -> Result<bool> {
    Ok(!git(&["log", &format!("origin/{}..HEAD", branch)], path)?.is_empty())
}

fn get_email(path: &Path) -> Result<String> {
    Ok(
        match git(&["config", "--get", "user.email"], path)?.strip_suffix(NEWLINE) {
            Some(s) => s.to_string(),
            None => NONE.to_string(),
        },
    )
}

fn git(args: &[&str], wd: &Path) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(wd).output()?;
    Ok(String::from_utf8(output.stdout)?)
}
