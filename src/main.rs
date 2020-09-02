/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

#[macro_use]
extern crate prettytable;

use structopt::StructOpt;

use std::env::current_dir;
use std::path::PathBuf;

mod gfold;
mod util;
use gfold::harness;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gfold",
    about = "https://github.com/nickgerace/gfold\n\n\
This application helps your organize multiple Git repositories via CLI.\n\
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

fn main() {
    let mut path = current_dir().expect("failed to get CWD");

    let opt = Opt::from_args();
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };

    path = path.canonicalize().expect("failed to canonicalize path");
    harness(&path, opt.recursive, opt.skip_sort);
}
