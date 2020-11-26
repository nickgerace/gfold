/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use crate::driver;

use std::path::{Path, PathBuf};

use eyre::Result;
use log::debug;

pub fn create_table_from_paths(
    repos: Vec<PathBuf>,
    path: &Path,
    no_color: &bool,
) -> Option<driver::TableWrapper> {
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
        let repo_obj = git2::Repository::open(&repo).ok()?;

        // FIXME: in case deeper recoverable errors are desired, use the match arm...
        // Err(error) if error.class() == git2::ErrorClass::Config => continue,
        let origin = match repo_obj.find_remote("origin") {
            Ok(origin) => origin,
            Err(_) => continue,
        };
        let url = match origin.url() {
            Some(url) => url,
            None => "none",
        };

        let head = repo_obj.head().ok()?;
        let branch = match head.shorthand() {
            Some(branch) => branch,
            None => "none",
        };

        let str_name = match Path::new(&repo).strip_prefix(path).ok()?.to_str() {
            Some(x) => x,
            None => "none",
        };

        if repo_obj.is_bare() {
            if *no_color {
                table.add_row(row![Fl->str_name, Fl->"bare", Fl->branch, Fl->url]);
            } else {
                table.add_row(row![Flb->str_name, Frl->"bare", Fl->branch, Fl->url]);
            }
        } else {
            let mut opts = git2::StatusOptions::new();
            match repo_obj.statuses(Some(&mut opts)) {
                Ok(statuses) if statuses.is_empty() => {
                    if is_unpushed(&repo_obj, &head).ok()? {
                        if *no_color {
                            table.add_row(row![Fl->str_name, Fl->"unpushed", Fl->branch, Fl->url])
                        } else {
                            table.add_row(row![Flb->str_name, Fcl->"unpushed", Fl->branch, Fl->url])
                        }
                    } else if *no_color {
                        table.add_row(row![Fl->str_name, Fl->"clean", Fl->branch, Fl->url])
                    } else {
                        table.add_row(row![Flb->str_name, Fgl->"clean", Fl->branch, Fl->url])
                    }
                }
                Ok(_) => {
                    if *no_color {
                        table.add_row(row![Fl->str_name, Fl->"unclean", Fl->branch, Fl->url])
                    } else {
                        table.add_row(row![Flb->str_name, Fyl->"unclean", Fl->branch, Fl->url])
                    }
                }
                Err(_) => {
                    if *no_color {
                        table.add_row(row![Fl->str_name, Fl->"error", Fl->branch, Fl->url])
                    } else {
                        table.add_row(row![Flb->str_name, Frl->"error", Fl->branch, Fl->url])
                    }
                }
            };
        }
    }

    match table.is_empty() {
        true => None,
        false => Some(driver::TableWrapper {
            path_string: path.to_str()?.to_string(),
            table,
        }),
    }
}

fn is_unpushed(repo: &git2::Repository, head: &git2::Reference) -> Result<bool> {
    let local = head.peel_to_commit()?;
    debug!("Local commit: {:#?}", local.id());

    let upstream = repo
        .resolve_reference_from_short_name("origin")?
        .peel_to_commit()?;
    debug!("Origin commit: {:#?}", upstream.id());

    match repo.graph_ahead_behind(local.id(), upstream.id())? {
        ahead if ahead.0 > 0 => Ok(true),
        _ => Ok(false),
    }
}
