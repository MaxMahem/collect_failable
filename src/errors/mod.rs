#[cfg(feature = "arrayvec")]
mod exceeds_capacity;
mod item_count_mismatch;
mod collection_collision;
mod key_collision;
mod one_of2;
mod collection_error;
mod value_collision;

#[cfg(feature = "arrayvec")]
pub use exceeds_capacity::*;
pub use item_count_mismatch::*;
pub use collection_collision::*;
pub use key_collision::*;
pub use one_of2::*;
pub use collection_error::*;
pub use value_collision::*;
