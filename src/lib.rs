#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![allow(clippy::match_bool)]
#![allow(clippy::single_match_else)]
//#![no_std]

/// Error types returned by provided failable collection operation implementations.
pub mod errors;
mod impls;
mod traits;

pub use traits::*;

#[cfg(feature = "tuple")]
/// Re-export of the `Either` type from the `either` crate.
pub mod either {
    pub use either::Either;
}
