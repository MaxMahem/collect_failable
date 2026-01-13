//! Error types returned by failable collection operations.
//!
//! This module provides a comprehensive set of error types designed to preserve as much
//! information as possible when collection operations fail.
//!
//! - The partial collection that was built before the error
//! - The item that caused the failure (if applicable)
//! - The remaining iterator that wasn't consumed
//!
//! ## Error Type Categories
//!
//! ### Simple Errors
//!
//! - [`ItemCollision`] - Single item that couldn't be added to a collection
//! - [`CapacityMismatch`] - Capacity constraint violations with [`MismatchKind`] details
//!
//! These are lightweight errors with no complex state management.
//!
//! ### Iterator Recovery Errors (require `alloc` feature)
//!
//! These errors preserve iteration state for full data recovery. All three share:
//! - Deref access to fields (read-only)
//! - `into_data()` for owned field access
//!
//! - [`CollectionCollision`] - Key/value collision during iteration
//! - [`CollectionError`] - Wraps another error `E` with partial collection state
//!
//! Both implement `IntoIterator` to reconstruct the original iterator.
//!
//! - [`ResultCollectionError`] - Dual error sources (iterator error + collection error)
//!
//! ### Tuple Errors (require `tuple` feature)
//!
//! - [`TupleExtensionError`] - Errors from extending tuple collections
//! - [`UnzipError`] - Errors from unzipping operations
//!
//! Both use `Either` to represent which side of the tuple failed.
//!
//! ## Read-Only API with `into_data`
//!
//! Iterator recovery errors use a **read-only public API** design pattern. Fields are accessible
//! via a hidden `Deref` implementation, but the error itself cannot be mutated or destructured:
//!
//! ```rust
//! # use collect_failable::{TryFromIterator, errors::CollectionCollision};
//! # use std::collections::HashMap;
//! let result = HashMap::try_from_iter([(1, 2), (1, 3), (2, 4)]);
//!
//! if let Err(err) = result {
//!     // ✅ Read fields via Deref
//!     let _collected: &HashMap<_, _> = &err.collected;
//!     let _item: &(i32, i32) = &err.item;
//!     
//!     // ✅ Reconstruct the original iterator via IntoIterator
//!     let recovered: Vec<_> = err.into_iter().collect();
//!     assert_eq!(recovered.len(), 3);
//!     
//!     // ✅ Or consume to get owned data via into_data()
//!     // let data = err.into_data();
//!     // Now you own: data.iterator, data.collected, data.item
//! }
//! ```
//!
//! ## Re-exports
//!
//! This module re-exports [`SizeHint`](size_hinter::SizeHint) from the `size_hinter` crate,
//! which is used by several error types to represent iterator bounds.
mod capacity_mismatch;
#[cfg(feature = "alloc")]
mod collection_collision;
#[cfg(feature = "alloc")]
mod collection_error;
mod item_collision;
#[cfg(feature = "alloc")]
mod result_collection_error;

#[cfg(feature = "tuple")]
mod tuple_extension_error;
#[cfg(feature = "tuple")]
mod unzip_error;

pub use capacity_mismatch::*;
#[cfg(feature = "alloc")]
pub use collection_collision::*;
#[cfg(feature = "alloc")]
pub use collection_error::*;
pub use item_collision::*;
#[cfg(feature = "alloc")]
pub use result_collection_error::*;

#[cfg(feature = "tuple")]
pub use tuple_extension_error::*;
#[cfg(feature = "tuple")]
pub use unzip_error::*;

pub use size_hinter::SizeHint;
