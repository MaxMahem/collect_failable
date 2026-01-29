//! Error types returned by failable collection operations.

/// Error and traits relating to collections with fixed capacity.
pub mod capacity;

/// Errors relating to collections with collisions.
pub mod collision;

mod error_item_provider;

#[cfg(feature = "alloc")]
mod collect_error;
#[cfg(feature = "alloc")]
mod extend_error;
#[cfg(feature = "alloc")]
mod result_collection_error;

#[cfg(feature = "tuple")]
mod tuple_extend_error;
#[cfg(feature = "tuple")]
mod unzip_error;

pub use collision::*;
pub use error_item_provider::*;

#[cfg(feature = "alloc")]
pub use collect_error::*;
#[cfg(feature = "alloc")]
pub use extend_error::*;
#[cfg(feature = "alloc")]
pub use result_collection_error::*;

#[cfg(feature = "tuple")]
pub use tuple_extend_error::*;
#[cfg(feature = "tuple")]
pub use unzip_error::*;

/// Foreign types used by the error types.
pub mod types {
    /// Re-export of the `Either` type from the `either` crate.
    #[cfg(feature = "tuple")]
    pub use either::Either;

    /// Re-export of the [`SizeHint`] type from the `size_hinter` crate.
    pub use size_hinter::SizeHint;
}

/// Types related to [`PartialArray`](crate::impls::unsafe::PartialArray).
#[cfg(feature = "unsafe")]
pub mod partial_array {
    pub use crate::impls::r#unsafe::{Drain, IntoArrayError, PartialArray};
}
