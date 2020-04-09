/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: MIT License
 */

use colored::Colorize;
use git2::{Repository, StatusOptions};

use std::fs;
use std::path::{Path, PathBuf};

pub fn walk_dir(path: &Path) {
    let working = &path;
    let paths = match fs::read_dir(&path) {
        Ok(paths) => paths,
        Err(e) => panic!("failed to get sub directories: {}", e),
    };

    let mut repos = Vec::new();
    let mut max: usize = 0;
    for item in paths {
        let temp = match &item {
            Ok(temp) => temp.path(),
            Err(e) => panic!("failed to get DirEntry: {}", e),
        };
        if temp.as_path().is_dir() && is_git_repo(temp.as_path()) {
            let directory = format!("{}", temp.display());
            let length = directory.chars().count();
            if length > max {
                max = length;
            }
            repos.push(directory)
        }
    }

    if !repos.is_empty() {
        repos.sort();
        for repo in repos {
            let spaces: usize = max - repo.chars().count();
            print_status(repo, spaces, working.to_path_buf());
        }
    }
}

fn is_git_repo(target: &Path) -> bool {
    let repo = match Repository::open(target) {
        Ok(_repo) => true,
        Err(_e) => false,
    };
    return repo;
}

fn print_status(directory: String, spaces: usize, path: PathBuf) {
    let repo = match Repository::open(&directory) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let origin = match repo.find_remote("origin") {
        Ok(origin) => origin,
        Err(e) => panic!("failed get origin: {}", e),
    };
    let url = match origin.url() {
        Some(url) => url,
        None => "none",
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(e) => panic!("failed get head: {}", e),
    };
    let branch = match head.shorthand() {
        Some(head) => head,
        None => "none",
    };

    let mut opts = StatusOptions::new();
    let statuses = match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => statuses,
        Err(e) => panic!("failed get statuses: {}", e),
    };

    let formatted_name = match Path::new(&directory).strip_prefix(path) {
        Ok(formatted_name) => formatted_name,
        Err(e) => panic!("failed to format name from Path object: {}", e),
    };
    let name_as_string = format!("{}", formatted_name.display());

    let status_message = if statuses.is_empty() {
        "clean  ".green()
    } else {
        "unclean".yellow()
    };
    println!(
        "{}{}  {}  {}  {}",
        name_as_string.bold(),
        " ".repeat(spaces),
        status_message,
        branch,
        url
    );
}

#[cfg(test)]
mod tests {
    use super::print_status;
    use std::env::current_dir;

    #[test]
    fn self_test() {
        let mut current_dir = match current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => panic!("failed to get CWD: {}", e),
        };
        let repo = format!("{}", current_dir.display());
        let parent = &mut current_dir;
        parent.pop();
        print_status(repo, 0, parent.to_path_buf());
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
