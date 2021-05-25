use anyhow::Result;
use std::{env, path::PathBuf};
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
    #[structopt(short, long, help = "Only search in the target directory")]
    shallow: bool,
    #[structopt(
        short = "m",
        long = "show-email",
        help = "Toggle to show git config user.email"
    )]
    show_email: bool,
    #[structopt(short = "x", long, help = "Toggle to skip sorting")]
    skip_sort: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut path = env::current_dir()?;
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };

    gfold::driver::Driver::new(
        &path.canonicalize()?,
        gfold::driver::Config {
            enable_unpushed_check: opt.enable_unpushed_check,
            include_non_repos: opt.include_non_repos,
            no_color: opt.no_color,
            shallow: opt.shallow,
            show_email: opt.show_email,
            skip_sort: opt.skip_sort,
        },
    )?
    .print_results()?;
    Ok(())
}
