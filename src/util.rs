/*
 * gfold
 * https://github.com/nickgerace/gfold
 * Author: Nick Gerace
 * License: Apache 2.0
 */

use git2::Repository;

use std::path::Path;

pub fn is_git_repo(target: &Path) -> bool {
    match Repository::open(target) {
        Ok(_) => true,
        Err(_) => false,
    }
}
