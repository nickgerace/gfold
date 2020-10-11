/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use std::env;
use std::path::PathBuf;
use std::process;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gfold",
    about = "https://github.com/nickgerace/gfold\n\n\
This application helps you keep track of multiple Git repositories via CLI.\n\
By default, it displays relevant information for all repos in the current\n\
working directory."
)]
struct Opt {
    #[structopt(parse(from_os_str), help = "Target a different directory")]
    path: Option<PathBuf>,
    #[structopt(short, long, help = "Search recursively")]
    recursive: bool,
    #[structopt(short, long, help = "Toggle to skip sorting")]
    skip_sort: bool,
}

/// This file, ```main.rs```, serves as the primary driver for the ```gfold``` library.
/// It is intended to be used as a command-line interface.
fn main() {
    let mut path = env::current_dir().expect("failed to get CWD");

    let opt = Opt::from_args();
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };
    path = path.canonicalize().expect("failed to canonicalize path");

    if let Err(error) = gfold::run(&path, opt.recursive, opt.skip_sort) {
        eprintln!("Encountered fatal error: {}", error);
        process::exit(1);
    };
}
