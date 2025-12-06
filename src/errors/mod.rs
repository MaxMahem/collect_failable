#[cfg(feature = "arrayvec")]
mod exceeds_capacity;
mod item_count_mismatch;
mod key_collision;
mod one_of2;
mod value_collision;
mod partial_iter_error;

#[cfg(feature = "arrayvec")]
pub use exceeds_capacity::*;
pub use item_count_mismatch::*;
pub use key_collision::*;
pub use one_of2::*;
pub use value_collision::*;
pub use partial_iter_error::*;
