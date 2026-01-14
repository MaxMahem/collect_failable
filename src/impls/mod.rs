#[cfg(feature = "unsafe")]
mod r#unsafe;

#[cfg(feature = "arrayvec")]
mod arrayvec;

mod maps;
mod sets;

#[cfg(feature = "alloc")]
mod result;
#[cfg(feature = "alloc")]
mod vector;

#[cfg(feature = "tuple")]
mod tuples;
