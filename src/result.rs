//! This module contains the [`crate::result::Result`] type.

use crate::error::Error;

/// Generic [`std::result::Result`] wrapper around [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
