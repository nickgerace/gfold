//! This library drives [`gfold`](https://github.com/nickgerace/gfold), a CLI tool to help keep track of your Git repositories.
pub mod driver;
mod driver_internal;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn current_directory() {
        let current_dir = env::current_dir().expect("failed to get CWD");
        assert_ne!(
            driver::Driver::new(
                &current_dir,
                &driver::Config {
                    enable_unpushed_check: false,
                    include_non_repos: false,
                    no_color: false,
                    shallow: false,
                    show_email: false,
                    skip_sort: false,
                }
            )
            .is_err(),
            true
        );
    }

    #[test]
    fn parent_directory() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        assert_ne!(
            driver::Driver::new(
                &current_dir,
                &driver::Config {
                    enable_unpushed_check: false,
                    include_non_repos: false,
                    no_color: false,
                    shallow: false,
                    show_email: false,
                    skip_sort: false,
                }
            )
            .is_err(),
            true
        );
    }

    #[test]
    fn parent_directory_all_options() {
        let mut current_dir = env::current_dir().expect("failed to get CWD");
        current_dir.pop();
        let mut count = 1;
        for include_non_repos in vec![true, false] {
            for no_color in vec![true, false] {
                for shallow in vec![true, false] {
                    for show_email in vec![true, false] {
                        for skip_sort in vec![true, false] {
                            println!("[test:{} / include_non_repos:{} / no_color:{} / shallow:{} / show_email:{} / skip_sort:{}]", count, include_non_repos, no_color, shallow, show_email, skip_sort);
                            assert_ne!(
                                driver::Driver::new(
                                    &current_dir,
                                    &driver::Config {
                                        enable_unpushed_check: false,
                                        include_non_repos,
                                        no_color,
                                        shallow,
                                        show_email,
                                        skip_sort,
                                    }
                                )
                                .is_err(),
                                true
                            );
                            count += 1;
                        }
                    }
                }
            }
        }
    }
}
