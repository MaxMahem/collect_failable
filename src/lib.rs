#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::match_bool)]
#![allow(clippy::single_match_else)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod errors;
mod impls;
mod traits;

pub use traits::*;

#[cfg(feature = "tuple")]
/// Re-export of the `Either` type from the `either` crate.
pub mod either {
    pub use either::Either;
}

/// Re-export of the `SizeHint` type from the `size_hinter` crate.
pub use size_hinter::SizeHint;
