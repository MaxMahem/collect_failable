/// Error types returned by failable collection operations.
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
