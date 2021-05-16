use crate::internal_types;
use log::{debug, warn};
use prettytable::{Cell, Row};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Condition {
    Bare,
    Clean,
    Error,
    Unclean,
    Unpushed,
}

pub fn create_table_from_paths(
    repos: Vec<PathBuf>,
    non_repos: Vec<PathBuf>,
    path: &Path,
    enable_unpushed_check: &bool,
    no_color: &bool,
    show_email: &bool,
) -> Option<internal_types::TableWrapper> {
    let mut table = prettytable::Table::new();
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 1)
            .build(),
    );

    // FIXME: maximize error recovery in this loop.
    for repo in repos {
        debug!("Creating row from path: {:#?}", repo);
        let repo_obj = match git2::Repository::open(&repo) {
            Ok(repo) => repo,
            Err(_) => {
                debug!("Could not open Git repository. Continuing to next repository...");
                continue;
            }
        };

        // FIXME: in case deeper recoverable errors are desired, use the match arm...
        // Err(error) if error.class() == git2::ErrorClass::Config => continue,
        let origin = match repo_obj.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => {
                debug!("Could not find remote origin. Continuing to next repository...");
                continue;
            }
        };
        let url = origin.url().unwrap_or("none");
        debug!("[+] url: {:#?}", url);

        let head = repo_obj.head().ok()?;
        let branch = head.shorthand().unwrap_or("none");
        debug!("[+] branch: {:#?}", branch);

        // FIXME: test using the "is_bare()" method for a repository object.
        let mut opts = git2::StatusOptions::new();
        let condition = match repo_obj.statuses(Some(&mut opts)) {
            Ok(statuses) if statuses.is_empty() => {
                if *enable_unpushed_check && is_unpushed(&repo_obj, &head) {
                    Condition::Unpushed
                } else {
                    Condition::Clean
                }
            }
            Ok(_) => Condition::Unclean,
            Err(error)
                if error.code() == git2::ErrorCode::BareRepo
                    && error.class() == git2::ErrorClass::Repository =>
            {
                Condition::Bare
            }
            Err(_) => Condition::Error,
        };

        let name = repo.as_path().file_name()?.to_str()?;
        let create_row = |status_spec: &str, status: &str| -> prettytable::Row {
            let mut cells = vec![
                Cell::new(name).style_spec(if *no_color { "Fl" } else { "Flb" }),
                Cell::new(status).style_spec(if *no_color { "Fl" } else { status_spec }),
                Cell::new(branch).style_spec("Fl"),
                Cell::new(url).style_spec("Fl"),
            ];
            if *show_email {
                cells.insert(
                    cells.len(),
                    Cell::new(get_email(&repo).as_str()).style_spec("Fl"),
                );
            }
            Row::new(cells)
        };

        match condition {
            Condition::Bare => table.add_row(create_row("Frl", "bare")),
            Condition::Clean => table.add_row(create_row("Fgl", "clean")),
            Condition::Unclean => table.add_row(create_row("Fyl", "unclean")),
            Condition::Unpushed => table.add_row(create_row("Fcl", "unpushed")),
            _ => table.add_row(create_row("Frl", "error")),
        };
        debug!("[+] condition: {:#?}", condition);
    }

    // D.R.Y. is important, but the "non_repos" loop would not benefit from the closure used by
    // the "repos" loop. The "repos" loop's closure leverages variables in the loop's scope,
    // whereas the "non_repos" loop does not create any local variables beyond the row's cells.
    for non_repo in non_repos {
        let mut cells = vec![
            Cell::new(non_repo.as_path().file_name()?.to_str()?).style_spec(if *no_color {
                "Fl"
            } else {
                "Flb"
            }),
            Cell::new("dir").style_spec(if *no_color { "Fl" } else { "Fml" }),
            Cell::new("-").style_spec("Fl"),
            Cell::new("-").style_spec("Fl"),
        ];
        if *show_email {
            cells.insert(cells.len(), Cell::new("-").style_spec("Fl"));
        }
        table.add_row(Row::new(cells));
    }

    debug!("Generated {:#?} rows for table object", table.len());
    match table.is_empty() {
        true => None,
        false => Some(internal_types::TableWrapper {
            path_string: path.to_str()?.to_string(),
            table,
        }),
    }
}

// FIXME: this function may not currently work because "clean", non-main branches can be considered "unpushed".
fn is_unpushed(repo: &git2::Repository, head: &git2::Reference) -> bool {
    let local = match head.peel_to_commit() {
        Ok(local) => local,
        Err(e) => {
            debug!("[-] error: {}", e);
            return false;
        }
    };
    debug!("[+] local commit: {:#?}", local.id());
    if let Some(name) = head.name() {
        debug!("[+] local ref: {}", name);
    }

    let upstream = match repo.resolve_reference_from_short_name("origin") {
        Ok(reference) => {
            if let Some(name) = reference.name() {
                debug!("[+] origin ref: {}", name);
            }
            match reference.peel_to_commit() {
                Ok(upstream) => upstream,
                Err(e) => {
                    debug!("[-] error: {}", e);
                    return false;
                }
            }
        }
        Err(e) => {
            debug!("[-] error: {}", e);
            return false;
        }
    };
    debug!("[+] origin commit: {:#?}", upstream.id());

    matches!(repo.graph_ahead_behind(local.id(), upstream.id()), Ok(ahead) if ahead.0 > 0)
}

fn get_email(repo_path: &Path) -> String {
    let func = |cfg: git2::Config| -> Option<String> {
        let entries = match cfg.entries(None) {
            Ok(o) => o,
            Err(e) => {
                debug!("Encountered error. Returning none: {:#?}", e);
                return None;
            }
        };
        for entry in &entries {
            let entry = match entry {
                Ok(o) => o,
                Err(e) => {
                    warn!("Encountered error: {:#?}", e);
                    continue;
                }
            };
            let key = match entry.name() {
                Some(s) => s,
                None => continue,
            };
            if key == "user.email" {
                let c = entry.value();
                match c {
                    Some(s) => return Some(s.to_string()),
                    None => continue,
                }
            }
        }
        None
    };

    match git2::Config::open(&repo_path.join(".git").join("config")) {
        Ok(o) => match func(o) {
            Some(value) => return value,
            None => debug!("Email not found. Trying default config..."),
        },
        Err(e) => debug!(
            "Encountered error accessing config in .git/config for {:#?}: {:#?}",
            &repo_path, e
        ),
    };
    match git2::Config::open_default() {
        Ok(o) => match func(o) {
            Some(value) => return value,
            None => debug!("Email not found in neither the default config nor the local config."),
        },
        Err(e) => debug!(
            "Encountered error accessing default git config for {:#?}: {:#?}",
            &repo_path, e
        ),
    };
    "-".to_string()
}
