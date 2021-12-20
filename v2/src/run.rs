use crate::{
    color,
    consts::{NEWLINE, PAD},
    error::Error,
    types::{Report, Status, Targets},
};
use anyhow::Result;
use rayon::prelude::*;
use std::{collections::BTreeMap, env, path::Path, process::Command};

pub fn run(provided_path: Option<&str>) -> Result<()> {
    let path = match provided_path {
        None => env::current_dir()?.canonicalize()?,
        Some(s) => env::current_dir()?.join(s).canonicalize()?,
    };

    let mut targets = Targets(Vec::new());
    targets.generate_targets(path)?;

    let unprocessed = targets
        .0
        .into_par_iter()
        .map(|m| process(&m))
        .collect::<Vec<Result<Report>>>();

    // Use a BTreeMap over a HashMap for sorted keys.
    let mut processed: BTreeMap<String, Vec<Report>> = BTreeMap::new();
    for wrapped_report in unprocessed {
        match wrapped_report {
            Ok(o) => {
                let report = o.clone();
                if let Some(mut s) = processed.insert(report.clone().parent, vec![report.clone()]) {
                    s.push(report.clone());
                    processed.insert(report.parent, s);
                }
            }
            Err(e) => return Err(e),
        }
    }
    print(&processed)
}

fn print(processed: &BTreeMap<String, Vec<Report>>) -> Result<()> {
    let length = processed.keys().len();
    let mut first = true;
    for group in processed {
        // FIXME: make group title matching less cumbersome.
        if length > 1 {
            match first {
                true => {
                    first = false;
                }
                false => println!(),
            }
            color::write_group_title(group.0)?;
        }

        let mut path_max = 0;
        let mut branch_max = 0;
        let mut status_max = 0;
        for report in group.1 {
            if report.path.len() > path_max {
                path_max = report.path.len();
            }
            let status_length = report.status_as_string.len();
            if status_length > status_max {
                status_max = status_length;
            }
            if report.branch.len() > branch_max {
                branch_max = report.branch.len();
            }
        }

        let mut reports = group.1.clone();
        reports.sort_by(|a, b| a.path.cmp(&b.path));
        reports.sort_by(|a, b| a.status_as_string.cmp(&b.status_as_string));

        for report in reports {
            print!("{:<path_width$}", report.path, path_width = path_max + PAD);
            color::write_status(&report.status, status_max + PAD)?;
            println!(
                "{:<branch_width$}{}",
                report.branch,
                report.url,
                branch_width = branch_max + PAD
            );
        }
    }
    Ok(())
}

fn process(path: &Path) -> Result<Report> {
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
    let status_as_string = format!("{:?}", &status);

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
            None => "-".to_string(),
        },
        status,
        status_as_string,
        branch: branch.to_string(),
        url: match git(&["config", "remote.origin.url"], path)?.strip_suffix(NEWLINE) {
            Some(s) => s.to_string(),
            None => "-".to_string(),
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

fn git(args: &[&str], wd: &Path) -> Result<String> {
    match Command::new("git").args(args).current_dir(wd).output() {
        Ok(o) => match String::from_utf8(o.stdout) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.into()),
        },
        Err(e) => Err(e.into()),
    }
}
