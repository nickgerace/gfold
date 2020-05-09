/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: MIT License
 */

use git2::{ErrorClass, Repository, StatusOptions};
use prettytable::{format, Table};

use std::fs;
use std::path::Path;

pub fn walk_dir(path: &Path) {
    let paths = match fs::read_dir(&path) {
        Ok(paths) => paths,
        Err(e) => panic!("failed to get sub directories: {}", e),
    };

    let mut repos = Vec::new();
    for item in paths {
        let temp = match &item {
            Ok(temp) => temp.path(),
            Err(e) => panic!("failed to get DirEntry: {}", e),
        };
        if temp.as_path().is_dir() && is_git_repo(temp.as_path()) {
            repos.push(temp.as_path().to_owned());
        }
    }

    let mut table = Table::new();
    table.set_format(
        format::FormatBuilder::new()
            .column_separator(' ')
            .padding(0, 1)
            .build(),
    );

    // TODO: Make this an asychronous function and sort afterwards.
    if !repos.is_empty() {
        repos.sort();
        for repo in repos {
            let repo_obj = match Repository::open(&repo) {
                Ok(repo_obj) => repo_obj,
                Err(e) => panic!("failed to open: {}", e),
            };

            // This match cascade combats the error: remote 'origin' does not exist.
            let origin = match repo_obj.find_remote("origin") {
                Ok(origin) => origin,
                Err(e) => match e.class() {
                    ErrorClass::Config => continue,
                    e => panic!("{:?}", e),
                },
            };
            let url = match origin.url() {
                Some(url) => url,
                None => "none",
            };
            let head = match repo_obj.head() {
                Ok(head) => head,
                Err(e) => panic!("failed get head: {}", e),
            };
            let branch = match head.shorthand() {
                Some(head) => head,
                None => "none",
            };
            let mut opts = StatusOptions::new();
            let statuses = match repo_obj.statuses(Some(&mut opts)) {
                Ok(statuses) => statuses,
                Err(e) => panic!("failed get statuses: {}", e),
            };
            let formatted_name = match Path::new(&repo).strip_prefix(path) {
                Ok(formatted_name) => formatted_name,
                Err(e) => panic!("failed to format name from Path object: {}", e),
            };
            let str_name = match formatted_name.to_str() {
                Some(x) => x,
                None => "none",
            };

            if statuses.is_empty() {
                table.add_row(row![Flb->str_name, Fgl->"clean", Fl->branch, Fl->url]);
            } else {
                table.add_row(row![Flb->str_name, Fyl->"unclean", Fl->branch, Fl->url]);
            };
        }
    }

    table.printstd();
}

fn is_git_repo(target: &Path) -> bool {
    let repo = match Repository::open(target) {
        Ok(_) => true,
        Err(_) => false,
    };
    return repo;
}

#[cfg(test)]
mod tests {
    use super::walk_dir;
    use std::env::current_dir;

    #[test]
    fn current_directory() {
        let current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => panic!("failed to get CWD: {}", e),
        };
        walk_dir(&current_dir);
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => panic!("failed to get CWD: {}", e),
        };
        current_dir.pop();
        walk_dir(&current_dir);
    }
}

/*
FIXME: this section is for the eventual recursive function. It's a proof of concept and is
not "safe" or "good" code yet.

if is_git_repo(Path::new(temp)) {
    // Print output or store in data sctructure.
} else {
    walk_dir(Path::new(temp), &depth + 1);
}

// Print the current directory, not the current working directory.
let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
println!("{}{}", " ".repeat(depth * 2), components.last().unwrap().to_str().unwrap().bold());

// If padding is used on the left-side of the terminal emulator, then add it here.
let pad = (depth * 2) + 2;
*/
