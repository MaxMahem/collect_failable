#![doc = include_str!("../README.md")]
mod collect_failable_ext;
mod try_from_iterator;

pub use collect_failable_ext::FailableCollectExt;
pub use try_from_iterator::TryFromIterator;

#[cfg(any(feature = "hash_map", feature = "btree_map"))]
pub use try_from_iterator::KeyCollision;
