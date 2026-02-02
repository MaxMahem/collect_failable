//! Error types returned by failable collection operations.

/// Error and traits relating to collections with fixed capacity.
pub mod capacity;

/// Errors relating to collections with collisions.
pub mod collision;

mod error_item_provider;

mod collect_error;
mod extend_error;
mod result_collection_error;

#[cfg(feature = "tuples")]
mod tuple_extend_error;
#[cfg(feature = "tuples")]
mod unzip_error;

pub use error_item_provider::*;

pub use collect_error::*;
pub use extend_error::*;
pub use result_collection_error::*;

pub use capacity::CapacityError;
pub use collision::Collision;

#[cfg(feature = "tuples")]
pub use tuple_extend_error::*;
#[cfg(feature = "tuples")]
pub use unzip_error::*;

/// Foreign types used by the error types.
pub mod types {
    /// Re-export of the `Either` type from the `either` crate.
    #[cfg(feature = "tuples")]
    pub use either::Either;

    /// Re-export of the [`SizeHint`] type from the `size_hinter` crate.
    pub use size_hinter::SizeHint;
}

/// Types related to [`PartialArray`](crate::impls::unsafe::PartialArray).
#[cfg(feature = "unsafe")]
pub mod partial_array {
    #[doc(hidden)]
    pub use crate::impls::r#unsafe::{ArrayIndex, PostInc};
    pub use crate::impls::r#unsafe::{Drain, IntoArrayError, PartialArray};
}
