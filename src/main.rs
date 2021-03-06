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
    #[structopt(short, long, help = "Set to debug mode")]
    debug: bool,
    #[structopt(
        short,
        long,
        help = "Toggle to enable checking for unpushed commits (experimental)"
    )]
    enable_unpushed_check: bool,
    #[structopt(short, long, help = "Include standard directories in the result")]
    include_non_repos: bool,
    #[structopt(short = "q", long = "nc", help = "Disable color output")]
    no_color: bool,
    #[structopt(parse(from_os_str), help = "Target a different directory")]
    path: Option<PathBuf>,
    #[structopt(short, long, help = "Search recursively")]
    recursive: bool,
    #[structopt(
        short = "m",
        long = "show-email",
        help = "Toggle to show git config user.email"
    )]
    show_email: bool,
    #[structopt(short, long, help = "Toggle to skip sorting")]
    skip_sort: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.debug {
        env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let mut path = env::current_dir()?;
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };
    path = path.canonicalize()?;

    gfold::run(
        &path,
        opt.enable_unpushed_check,
        opt.include_non_repos,
        opt.no_color,
        opt.recursive,
        opt.show_email,
        opt.skip_sort,
    )?;
    Ok(())
}
