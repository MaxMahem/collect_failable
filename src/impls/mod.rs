mod ensure_empty;

#[doc(hidden)]
pub use ensure_empty::*;

#[cfg(feature = "unsafe")]
pub mod r#unsafe;

#[cfg(feature = "arrayvec")]
mod arrayvec;

mod maps;
mod sets;

#[cfg(feature = "alloc")]
mod result;

#[cfg(feature = "alloc")]
mod vec;

#[cfg(feature = "tuples")]
mod tuples;

mod macros;
