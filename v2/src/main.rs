use anyhow::Result;
use std::env;

mod color;
mod consts;
mod dir;
mod error;
mod run;
mod types;

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "off");
    }
    env_logger::init();

    match env::args().nth(1).as_deref() {
        Some(s) if s == "-h" || s == "--help" => {
            println!("gfold {}{}", consts::VERSION, consts::HELP);
            Ok(())
        }
        Some(s) if s == "-V" || s == "--version" => {
            println!("gfold {}", consts::VERSION);
            Ok(())
        }
        Some(s) => run::run(Some(s)),
        None => run::run(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn current_directory() {
        assert!(run::run(None).is_ok());
    }

    #[test]
    fn parent_directory() {
        let mut parent = env::current_dir().expect("failed to get CWD");
        parent.pop();
        assert!(run::run(Some(
            parent
                .to_str()
                .expect("found None for PathBuf conversion to &str")
        ))
        .is_ok());
    }

    #[test]
    fn home_directory() {
        assert!(run::run(Some(env!("HOME"))).is_ok());
    }
}
