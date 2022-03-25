//! This module contains the [`anyhow::Result`] type.

use crate::error::Error;

/// Generic [`anyhow::Result`] wrapper around [`Error`].
pub type Result<T> = anyhow::Result<T, Error>;
