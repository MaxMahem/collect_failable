//! Error types returned by failable collection operations.

mod capacity_error;
mod collision;
mod error_item_provider;

#[cfg(feature = "alloc")]
mod collect_error;
#[cfg(feature = "alloc")]
mod result_collection_error;

#[cfg(feature = "tuple")]
mod tuple_extension_error;
#[cfg(feature = "tuple")]
mod unzip_error;

pub use capacity_error::*;
pub use collision::*;
pub use error_item_provider::*;

#[cfg(feature = "alloc")]
pub use collect_error::*;
#[cfg(feature = "alloc")]
pub use result_collection_error::*;

#[cfg(feature = "tuple")]
pub use tuple_extension_error::*;
#[cfg(feature = "tuple")]
pub use unzip_error::*;

/// Re-export of the `Either` type from the `either` crate.
#[cfg(feature = "tuple")]
pub use either::Either;

/// Re-export of the [`SizeHint`] type from the `size_hinter` crate.
pub use size_hinter::SizeHint;

/// Types related to [`PartialArray`](crate::impls::unsafe::PartialArray).
#[cfg(feature = "unsafe")]
pub mod partial_array {
    pub use crate::impls::r#unsafe::{Drain, IntoArrayError, PartialArray};
}
