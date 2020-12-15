/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use crate::driver;

use std::path::{Path, PathBuf};

use log::debug;

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
        let url = match origin.url() {
            Some(url) => url,
            None => "none",
        };
        debug!("[+] url: {:#?}", url);

        let head = repo_obj.head().ok()?;
        let branch = match head.shorthand() {
            Some(branch) => branch,
            None => "none",
        };
        debug!("[+] branch: {:#?}", branch);

        let name = match Path::new(&repo).strip_prefix(path).ok()?.to_str() {
            Some(name) => name,
            None => "none",
        };
        debug!("[+] name: {:#?}", name);

        // FIXME: test using the "is_bare()" method for a repository object.
        let mut opts = git2::StatusOptions::new();
        let condition = match repo_obj.statuses(Some(&mut opts)) {
            Ok(statuses) if statuses.is_empty() => {
                if is_unpushed(&repo_obj, &head) {
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
        debug!("[+] condition: {:#?}", condition);
    }

    debug!("Generated {:#?} rows for table object", table.len());
    match table.is_empty() {
        true => None,
        false => Some(driver::TableWrapper {
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

    let upstream = match repo.resolve_reference_from_short_name("origin") {
        Ok(reference) => match reference.peel_to_commit() {
            Ok(upstream) => upstream,
            Err(e) => {
                debug!("[-] error: {}", e);
                return false;
            }
        },
        Err(e) => {
            debug!("[-] error: {}", e);
            return false;
        }
    };
    debug!("[+] origin commit: {:#?}", upstream.id());

    matches!(repo.graph_ahead_behind(local.id(), upstream.id()), Ok(ahead) if ahead.0 > 0)
}
