/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use std::env;
use std::path::PathBuf;

use eyre::Result;
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
    #[structopt(long = "nc", help = "Disable color output")]
    no_color: bool,
    #[structopt(parse(from_os_str), help = "Target a different directory")]
    path: Option<PathBuf>,
    #[structopt(short, long, help = "Search recursively")]
    recursive: bool,
    #[structopt(short, long, help = "Toggle to skip sorting")]
    skip_sort: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let opt = Opt::from_args();
    let mut path = env::current_dir()?;
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };
    path = path.canonicalize()?;

    gfold::run(&path, opt.no_color, opt.recursive, opt.skip_sort)?;
    Ok(())
}
