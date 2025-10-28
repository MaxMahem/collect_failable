#![doc = include_str!("../README.md")]
mod collect_failable_ext;
mod try_from_iterator;

pub use collect_failable_ext::FailableCollectExt;
pub use try_from_iterator::KeyCollision;
pub use try_from_iterator::TryFromIterator;
