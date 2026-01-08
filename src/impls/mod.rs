#[cfg(feature = "unsafe")]
mod r#unsafe;

#[cfg(feature = "arrayvec")]
mod arrayvec;

mod maps;
mod sets;

#[cfg(feature = "alloc")]
mod result;

#[cfg(feature = "tuple")]
mod tuples;
