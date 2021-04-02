use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut path = env::current_dir()?;
    match args.len() {
        args_length if args_length < 2 => gfld::run(&path.canonicalize()?)?,
        _ if args[1].as_str() == "-h" || args[1].as_str() == "--help" => println!(
            "gfld {}
https://github.com/nickgerace/gfld

Lists the status of all Git projects in a directory recursively.
Uses the current working directory by default, and can use a path as the first argument.

USAGE:
    gfld [path/-h/--help]",
            option_env!("CARGO_PKG_VERSION").unwrap_or("v?")
        ),
        _ => {
            path.push(args[1].clone());
            gfld::run(&path.canonicalize()?)?;
        }
    }
    Ok(())
}
