#![doc = include_str!("../README.md")]
mod impls;
mod try_collect_ex;
mod try_extend;
mod try_from_iterator;

pub use try_collect_ex::TryCollectEx;
pub use try_extend::{NonUniqueKey, TryExtend};
pub use try_from_iterator::{KeyCollision, TryFromIterator};
