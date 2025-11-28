#![doc = include_str!("../README.md")]

#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]

#![allow(clippy::match_bool)]
#![allow(clippy::single_match_else)]

// Allow multiple crate versions from transitive dependencies (include-doc)
#![allow(clippy::multiple_crate_versions)]

mod errors;
mod impls;
mod traits;

pub use errors::{ItemCountMismatch, KeyCollision, OneOf2, ValueCollision};
pub use traits::{FoldMut, TryCollectEx, TryExtend, TryFromIterator, TryUnzip};

#[cfg(feature = "arrayvec")]
pub use errors::ExceedsCapacity;
