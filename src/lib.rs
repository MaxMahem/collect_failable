#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::match_bool)]
#![allow(clippy::single_match_else)]
// Allow multiple crate versions from transitive dependencies (include-doc)
#![allow(clippy::multiple_crate_versions)]

/// Error types returned by provided failable collection operation implementations.
pub mod errors;
mod impls;
mod traits;

pub use traits::*;

#[cfg(feature = "tuple")]
/// Re-export of the `Either` type from the `either` crate.
pub mod either {
    #[doc(inline)]
    pub use either::Either;
}
