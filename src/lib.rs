#![doc = include_str!("../README.md")]
mod iterators;
mod try_collect_ex;
mod try_from_iterator;

pub use try_collect_ex::TryCollectEx;
pub use try_from_iterator::KeyCollision;
pub use try_from_iterator::TryFromIterator;
