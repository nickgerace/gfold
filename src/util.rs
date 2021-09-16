use crate::types::TableWrapper;
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
) -> Option<TableWrapper> {
    let mut table = prettytable::Table::new();
    table.set_format(
        prettytable::format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 1)
            .build(),
    );

    // FIXME: maximize error recovery in this loop.
    for repo in repos {
        let repo_obj = match git2::Repository::open(&repo) {
            Ok(repo) => repo,
            Err(_) => continue,
        };

        // FIXME: in case deeper recoverable errors are desired, use the match arm...
        // Err(error) if error.class() == git2::ErrorClass::Config => continue,
        let origin = match repo_obj.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => continue,
        };
        let url = origin.url().unwrap_or("none");
        let head = repo_obj.head().ok()?;
        let branch = head.shorthand().unwrap_or("none");

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
                Cell::new(name).style_spec("Fl"),
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
    }

    // D.R.Y. is important, but the "non_repos" loop would not benefit from the closure used by
    // the "repos" loop. The "repos" loop's closure leverages variables in the loop's scope,
    // whereas the "non_repos" loop does not create any local variables beyond the row's cells.
    for non_repo in non_repos {
        let mut cells = vec![
            Cell::new(non_repo.as_path().file_name()?.to_str()?).style_spec("Fl"),
            Cell::new("dir").style_spec(if *no_color { "Fl" } else { "Fml" }),
            Cell::new("-").style_spec("Fl"),
            Cell::new("-").style_spec("Fl"),
        ];
        if *show_email {
            cells.insert(cells.len(), Cell::new("-").style_spec("Fl"));
        }
        table.add_row(Row::new(cells));
    }

    match table.is_empty() {
        true => None,
        false => Some(TableWrapper {
            path_string: path.to_str()?.to_string(),
            table,
        }),
    }
}

// FIXME: this function may not currently work because "clean", non-main branches can be considered "unpushed".
fn is_unpushed(repo: &git2::Repository, head: &git2::Reference<'_>) -> bool {
    let local = match head.peel_to_commit() {
        Ok(local) => local,
        Err(_) => return false,
    };

    let upstream = match repo.resolve_reference_from_short_name("origin") {
        Ok(reference) => match reference.peel_to_commit() {
            Ok(upstream) => upstream,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    matches!(repo.graph_ahead_behind(local.id(), upstream.id()), Ok(ahead) if ahead.0 > 0)
}

fn get_email(repo_path: &Path) -> String {
    let find_email_in_config = |cfg: git2::Config| -> Option<String> {
        let entries = match cfg.entries(None) {
            Ok(o) => o,
            Err(_) => return None,
        };
        for entry in &entries {
            let entry = match entry {
                Ok(o) => o,
                Err(_) => continue,
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

    if let Ok(o) = git2::Config::open(&repo_path.join(".git").join("config")) {
        if let Some(o) = find_email_in_config(o) {
            return o;
        }
    }
    if let Ok(o) = git2::Config::open_default() {
        if let Some(o) = find_email_in_config(o) {
            return o;
        }
    }
    "-".to_string()
}
