//! This module contains the functionality for generating reports.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::Result;
use rayon::prelude::*;
use target::TargetCollector;

use crate::repository_view::RepositoryView;

mod target;

/// This type represents a [`BTreeMap`] using an optional [`String`] for keys, which represents the
/// parent directory for a group of reports ([`Vec<RepositoryView>`]). The values corresponding to those keys
/// are the actual groups of reports.
///
/// We use a [`BTreeMap`] instead of a [`HashMap`](std::collections::HashMap) in order to have
/// sorted keys.
pub type RepositoryCollection = BTreeMap<Option<String>, Vec<RepositoryView>>;

type UnprocessedRepositoryView = Result<RepositoryView>;

/// Generate [`RepositoryCollection`] for a given path and its children.
pub fn run(
    path: &Path,
    include_email: bool,
    include_submodules: bool,
) -> Result<RepositoryCollection> {
    let unprocessed = TargetCollector::run(path.to_path_buf())?
        .par_iter()
        .map(|path| RepositoryView::new(path, include_email, include_submodules))
        .collect::<Vec<UnprocessedRepositoryView>>();

    let mut processed = RepositoryCollection::new();
    for maybe_view in unprocessed {
        let view = maybe_view?;
        if let Some(mut views) = processed.insert(view.parent.clone(), vec![view.clone()]) {
            views.push(view.clone());
            processed.insert(view.parent, views);
        }
    }
    Ok(processed)
}
