#[cfg(feature = "unsafe")]
mod r#unsafe;

#[cfg(feature = "arrayvec")]
mod arrayvec;

mod maps;
mod result;
mod sets;

#[cfg(feature = "tuple")]
mod tuples;
