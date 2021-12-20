#[cfg(target_os = "windows")]
pub const NEWLINE: &str = "\r\n";
#[cfg(not(target_os = "windows"))]
pub const NEWLINE: &str = "\n";

pub const PAD: usize = 2;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const HELP: &str = "
https://github.com/nickgerace/gfold

This application helps you keep track of multiple Git repositories via CLI.
By default, it displays relevant information for all repos in the current
working directory.

USAGE:
    gfold [FLAGS] [path]

FLAGS:
    -h, --help                     Prints help information
    -V, --version                  Prints version information

ARGS:
    <path>    Target a different directory";
