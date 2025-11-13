#[cfg(feature = "hash_map")]
pub mod hash_map;

#[cfg(feature = "btree_map")]
pub mod btree_map;

#[cfg(feature = "hashbrown")]
pub mod hashbrown;

#[cfg(feature = "indexmap")]
pub mod indexmap;

#[cfg(test)]
mod tests;
