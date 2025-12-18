#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::match_bool)]
#![allow(clippy::single_match_else)]
// Allow multiple crate versions from transitive dependencies (include-doc)
#![allow(clippy::multiple_crate_versions)]

mod errors;
mod impls;
mod traits;

/// Helper utilities for testing and authoring failable collection implementations.
#[cfg(feature = "utils")]
pub mod utils;

pub use errors::*;
pub use traits::*;
