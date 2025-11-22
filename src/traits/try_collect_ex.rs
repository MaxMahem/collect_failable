use include_doc::function_body;

#[cfg(doc)]
use std::collections::HashMap;

use crate::TryFromIterator;

/// Extends [Iterator] with a failable collect method.
///
/// This trait lets you have an iterator return any collection that can be created via
/// [`TryFromIterator`], similar to [`Iterator::collect`] and [`FromIterator::from_iter`],
/// but with the ability to return a implementation specific error if the creation of the contaienr
/// fails some invariant.
pub trait TryCollectEx: Iterator {
    /// Tries to collects the iterator into a container, returning an error if construcing the
    /// container fails.
    ///
    /// Exact behavior of this method depends on the container implementation, but generally it
    /// should be expected to short-circuit on the first error.
    ///
    /// On success, this method should behave similarly to [`Iterator::collect`], except returning
    /// a [`Result`].
    ///
    /// Note: Ideally this would be called `try_collect` but there is a method with that name in nightly.
    ///
    /// # Errors
    ///
    /// Returns a [`TryFromIterator::Error`] if the container fails to be constructed.
    ///
    /// # Example
    ///
    /// Collecting into a [`HashMap`] that fails if a key would collide.
    ///
    /// ```rust
    #[doc = function_body!("tests/try_collect_ex.rs", try_collect_ex_collision_example, [])]
    /// ```
    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
        Self: Sized;
}

/// Implementation of [`TryCollectEx`] for all [`Iterator`].
impl<I, T> TryCollectEx for I
where
    I: Iterator<Item = T>,
{
    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item>,
    {
        C::try_from_iter(self)
    }
}
