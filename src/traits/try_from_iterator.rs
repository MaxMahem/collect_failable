use include_doc::function_body;

#[cfg(doc)]
use std::collections::HashMap;

#[cfg(doc)]
use crate::TryCollectEx;

/// Tries to convert a [`IntoIterator`] into a container that may fail to be constructed.
///
/// This trait is similar to [`FromIterator`], but can uphold a containers invariant and
/// returns an [`Err`] if it would be violated. And like with [`Iterator::collect`],
/// containers implementing this trait can be collected into via
/// [`TryCollectEx::try_collect_ex`].
///
/// Implementations for several common types are provided.
pub trait TryFromIterator<T>: Sized {
    /// The error that may occur when converting the iterator into the container.
    type Error;

    /// Tries to converts an iterator into a container that may fail to be constructed.
    ///
    /// # Errors
    ///
    /// Returns a [`TryFromIterator::Error`] error if the container fails to be constructed.
    ///
    /// Provided implementations all short-ciruit and error early if a constraint is violated,
    /// but implementors are not required to do so.
    ///
    /// # Example
    ///
    /// Provided [`HashMap`] implementations error if a key would collide.
    ///
    /// ```rust
    #[doc = function_body!("tests/try_from_iterator.rs", try_from_iter_collision_example, [])]
    /// ```    
    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error>;
}
