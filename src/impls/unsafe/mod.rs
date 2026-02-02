mod array;
mod array_index;
mod partial_array;

#[doc(hidden)]
pub use array_index::{ArrayIndex, PostInc};
pub use partial_array::{Drain, IntoArrayError, PartialArray};
