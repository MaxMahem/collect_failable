mod test_macros;

mod collection_collision;
mod collection_error;
mod result_collection_error;
mod tuple_collection_error;
mod tuple_extension_error;
mod unzip_error;

#[cfg(feature = "arrayvec")]
mod exceeds_capacity;
