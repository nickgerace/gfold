use crate::types::Opt;
use anyhow::Result;
use clap::Clap;
use std::env;

mod driver;
mod types;
mod util;

fn main() -> Result<()> {
    let opt = Opt::parse();

    let mut path = env::current_dir()?;
    if let Some(provided_path) = opt.path {
        path.push(provided_path)
    };

    driver::Driver::new(
        &path.canonicalize()?,
        opt.enable_unpushed_check,
        opt.include_non_repos,
        opt.no_color,
        opt.shallow,
        opt.show_email,
        opt.skip_sort,
    )?
    .print_results()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn current_directory() {
        let current_dir = env::current_dir().expect("failed to get CWD");
        assert!(
            !driver::Driver::new(&current_dir, false, false, false, false, false, false,).is_err()
        );
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert!(
            !driver::Driver::new(&current_dir, false, false, false, false, false, false,).is_err()
        );
    }

    #[test]
    fn parent_directory_all_options() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        let mut count = 1;
        for include_non_repos in &[true, false] {
            for no_color in &[true, false] {
                for shallow in &[true, false] {
                    for show_email in &[true, false] {
                        for skip_sort in &[true, false] {
                            println!("[test:{} / include_non_repos:{} / no_color:{} / shallow:{} / show_email:{} / skip_sort:{}]", count, include_non_repos, no_color, shallow, show_email, skip_sort);
                            assert!(!driver::Driver::new(
                                &current_dir,
                                false,
                                *include_non_repos,
                                *no_color,
                                *shallow,
                                *show_email,
                                *skip_sort,
                            )
                            .is_err());
                            count += 1;
                        }
                    }
                }
            }
        }
    }
}
