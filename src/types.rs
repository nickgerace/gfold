use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(
    name = "gfold",
    version = env!("CARGO_PKG_VERSION"),
    about = "https://github.com/nickgerace/gfold\n\n\
This application helps you keep track of multiple Git repositories via CLI.\n\
By default, it displays relevant information for all repos in the current\n\
working directory."
)]
pub struct Opt {
    #[clap(
        short,
        long,
        about = "Toggle to enable checking for unpushed commits (experimental)"
    )]
    pub enable_unpushed_check: bool,
    #[clap(short, long, about = "Include standard directories in the result")]
    pub include_non_repos: bool,
    #[clap(
        short = 'q',
        long = "nc",
        visible_alias = "no-color",
        about = "Disable color output"
    )]
    pub no_color: bool,
    #[clap(parse(from_os_str), about = "Target a different directory")]
    pub path: Option<PathBuf>,
    #[clap(short, long, about = "Only search in the target directory")]
    pub shallow: bool,
    #[clap(
        short = 'm',
        long = "show-email",
        about = "Toggle to show git config user.email"
    )]
    pub show_email: bool,
    #[clap(short = 'x', long, about = "Toggle to skip sorting")]
    pub skip_sort: bool,
}

pub struct TableWrapper {
    pub path_string: String,
    pub table: prettytable::Table,
}
