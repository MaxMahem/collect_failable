//! Error types returned by failable collection operations.

mod capacity_error;
mod collision;
mod error_item_provider;

#[cfg(feature = "alloc")]
mod collection_error;
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
pub use collection_error::*;
#[cfg(feature = "alloc")]
pub use result_collection_error::*;

#[cfg(feature = "tuple")]
pub use tuple_extension_error::*;
#[cfg(feature = "tuple")]
pub use unzip_error::*;
