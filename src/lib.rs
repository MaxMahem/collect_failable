#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod errors;
mod impls;
mod traits;

pub use traits::*;

/// Re-export of the `Either` type from the `either` crate.
#[cfg(feature = "tuple")]
pub mod either {
    pub use either::Either;
}

/// Re-export of the `SizeHint` type from the `size_hinter` crate.
pub use size_hinter::SizeHint;
