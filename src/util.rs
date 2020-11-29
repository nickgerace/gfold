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

enum Condition {
    Bare,
    Clean,
    Error,
    Unclean,
    Unpushed,
}

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

        let name = match Path::new(&repo).strip_prefix(path).ok()?.to_str() {
            Some(x) => x,
            None => "none",
        };

        let condition;
        if repo_obj.is_bare() {
            condition = Condition::Bare;
        } else {
            let mut opts = git2::StatusOptions::new();
            condition = match repo_obj.statuses(Some(&mut opts)) {
                Ok(statuses) if statuses.is_empty() => {
                    if is_unpushed(&repo_obj, &head).ok()? {
                        Condition::Unpushed
                    } else {
                        Condition::Clean
                    }
                }
                Ok(_) => Condition::Unclean,
                Err(_) => Condition::Error,
            };
        }

        match condition {
            Condition::Bare if *no_color => {
                table.add_row(row![Fl->name, Fl->"bare", Fl->branch, Fl->url])
            }
            Condition::Bare => table.add_row(row![Flb->name, Frl->"bare", Fl->branch, Fl->url]),
            Condition::Clean if *no_color => {
                table.add_row(row![Fl->name, Fl->"clean", Fl->branch, Fl->url])
            }
            Condition::Clean => table.add_row(row![Flb->name, Fgl->"clean", Fl->branch, Fl->url]),
            Condition::Unclean if *no_color => {
                table.add_row(row![Fl->name, Fl->"unclean", Fl->branch, Fl->url])
            }
            Condition::Unclean => {
                table.add_row(row![Flb->name, Fyl->"unclean", Fl->branch, Fl->url])
            }
            Condition::Unpushed if *no_color => {
                table.add_row(row![Fl->name, Fl->"unpushed", Fl->branch, Fl->url])
            }
            Condition::Unpushed => {
                table.add_row(row![Flb->name, Fcl->"unpushed", Fl->branch, Fl->url])
            }
            _ if *no_color => table.add_row(row![Fl->name, Fl->"error", Fl->branch, Fl->url]),
            _ => table.add_row(row![Flb->name, Frl->"error", Fl->branch, Fl->url]),
        };
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

    // FIXME: there is a bug where the "origin" resolved here is from the main remote branch, and
    // not the remote of the local branch being tracked. We may need to check if a remote exists
    // for the local branch with this fix.
    let upstream = repo
        .resolve_reference_from_short_name("origin")?
        .peel_to_commit()?;
    debug!("Origin commit: {:#?}", upstream.id());

    match repo.graph_ahead_behind(local.id(), upstream.id())? {
        ahead if ahead.0 > 0 => Ok(true),
        _ => Ok(false),
    }
}
