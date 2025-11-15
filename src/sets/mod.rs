pub mod btree_set;
pub mod hash_set;

#[cfg(feature = "hashbrown")]
pub mod hashbrown;

#[cfg(feature = "indexmap")]
pub mod indexset;

#[cfg(test)]
mod tests;
