/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: MIT License
 */

#[macro_use]
extern crate prettytable;

use clap::App;

use std::env::current_dir;

mod gfold;
use gfold::walk_dir;

fn main() {
    let matches = App::new("gfold")
        .version("0.2.1")
        .about(
            "https://github.com/nickgerace/gfold\n\n\
            This application helps your organize multiple Git repositories via CLI.\n\
            By default, it displays relevant information for all repos in the current \
            working directory.",
        )
        .arg("-p, --path=[DIRECTORY] 'Target a different directory'")
        .get_matches();

    let mut path = match current_dir() {
        Ok(path) => path,
        Err(e) => panic!("failed to get CWD: {}", e),
    };
    if let Some(p) = matches.value_of("path") {
        path.push(p);
    }
    path = match path.canonicalize() {
        Ok(path) => path,
        Err(e) => panic!("failed to canonicalize path: {}", e),
    };
    walk_dir(&path);
}
