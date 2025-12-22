/// Error types returned by failable collection operations.
mod capacity_mismatch;
mod collection_collision;
mod collection_error;
mod item_collision;
mod result_collection_error;

#[cfg(feature = "tuple")]
mod tuple_extension_error;
#[cfg(feature = "tuple")]
mod unzip_error;

pub use capacity_mismatch::*;
pub use collection_collision::*;
pub use collection_error::*;
pub use item_collision::*;
pub use result_collection_error::*;

#[cfg(feature = "tuple")]
pub use tuple_extension_error::*;
#[cfg(feature = "tuple")]
pub use unzip_error::*;
