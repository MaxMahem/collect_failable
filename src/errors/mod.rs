mod collection_collision;
mod collection_error;
mod item_count_mismatch;
mod result_collection_error;
mod tuple_collection_error;
mod tuple_extension_error;
mod unzip_error;

#[cfg(feature = "arrayvec")]
mod exceeds_capacity;

pub use collection_collision::*;
pub use collection_error::*;
pub use item_count_mismatch::*;
pub use result_collection_error::*;
pub use tuple_collection_error::*;
pub use tuple_extension_error::*;
pub use unzip_error::*;

#[cfg(feature = "arrayvec")]
pub use exceeds_capacity::*;
