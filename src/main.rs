/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

#[macro_use]
extern crate prettytable;

use std::env::current_dir;
use std::path::PathBuf;
use structopt::StructOpt;

mod gfold;
use gfold::walk_dir;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gfold",
    about = "https://github.com/nickgerace/gfold\n\n\
This application helps your organize multiple Git repositories via CLI.\n\
By default, it displays relevant information for all repos in the current\n\
working directory."
)]
struct Opt {
    #[structopt(
        name = "path",
        help = "Target a different directory",
        parse(from_os_str)
    )]
    path: Option<PathBuf>,
}

fn main() {
    let mut path = match current_dir() {
        Ok(path) => path,
        Err(e) => panic!("failed to get CWD: {}", e),
    };

    let opt = Opt::from_args();
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };

    path = match path.canonicalize() {
        Ok(path) => path,
        Err(e) => panic!("failed to canonicalize path: {}", e),
    };
    walk_dir(&path);
}
