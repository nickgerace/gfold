//! This module contains the functionality for generating reports.

use rayon::prelude::*;
use std::collections::BTreeMap;
use std::path::Path;
use target::TargetCollector;

use crate::config::DisplayMode;
use crate::repository_view::RepositoryView;

mod target;

/// This type represents a [`BTreeMap`] using an optional [`String`] for keys, which represents the
/// parent directory for a group of reports ([`Vec<RepositoryView>`]). The values corresponding to those keys
/// are the actual groups of reports.
///
/// We use a [`BTreeMap`] instead of a [`HashMap`](std::collections::HashMap) in order to have
/// sorted keys.
pub type RepositoryCollection = BTreeMap<Option<String>, Vec<RepositoryView>>;

type UnprocessedRepositoryView = anyhow::Result<RepositoryView>;

/// A unit struct that provides [`Self::run()`], which is used to generated [`RepositoryCollection`].
pub struct RepositoryCollector;

impl RepositoryCollector {
    /// Generate [`RepositoryCollection`] for a given path and its children. The [`DisplayMode`] is
    /// required because any two display modes can require differing amounts of data to be
    /// collected.
    pub fn run(path: &Path, display_mode: DisplayMode) -> anyhow::Result<RepositoryCollection> {
        let (include_email, include_submodules) = match display_mode {
            DisplayMode::Classic => (false, false),
            DisplayMode::Json => (true, true),
            DisplayMode::Standard => (true, false),
        };

        let unprocessed = TargetCollector::run(path.to_path_buf())?
            .par_iter()
            .map(|path| RepositoryView::new(path, include_email, include_submodules))
            .collect::<Vec<UnprocessedRepositoryView>>();

        let mut processed = RepositoryCollection::new();
        for maybe_view in unprocessed {
            match maybe_view {
                Ok(view) => {
                    if let Some(mut views) =
                        processed.insert(view.parent.clone(), vec![view.clone()])
                    {
                        views.push(view.clone());
                        processed.insert(view.parent, views);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(processed)
    }
}
