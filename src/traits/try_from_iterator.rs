#[cfg(doc)]
use std::collections::HashMap;

/// Tries to convert an [`IntoIterator`] into a container that may fail to be constructed.
///
/// This trait is similar to [`FromIterator`], but can uphold a container's invariant and
/// returns an [`Err`] if it would be violated. And like with [`Iterator::collect`],
/// containers implementing this trait can be collected into via
/// [`TryCollectEx::try_collect_ex`].
///
/// Implementations may rely on [`Iterator::size_hint`] providing reliable bounds for the
/// number of elements in the iterator in order to optimize their implementations. A size hint
/// that provides incorrect bounds may cause panics, produce incorrect results, or produce a
/// result that violates container constraints, but must not result in undefined behavior.
///
/// Implementations are encouraged to return all the data consumed by the iterator, as well
/// as the partially consumed iterator on an error, but are not required to do so.
pub trait TryFromIterator<I: IntoIterator>: Sized {
    /// The error that may occur when converting the iterator into the container.
    type Error;

    /// Tries to convert an iterator into a container that may fail to be constructed.
    ///
    /// Provided implementations all short-circuit and error early if a constraint is violated,
    /// but implementors are not required to do so.
    ///
    /// # Errors
    ///
    /// Returns a [`TryFromIterator::Error`] error if the container fails to be constructed.
    ///
    /// # Example
    ///
    /// Provided [`HashMap`] implementations error if a key would collide.
    ///
    /// ```rust
    /// use collect_failable::TryFromIterator;
    /// use std::collections::HashMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// // Success case - type inference works from return type
    /// let map: HashMap<_, _> = HashMap::try_from_iter([(1, 2), (2, 3)]).expect("should succeed");
    /// assert_eq!(map.len(), 2);
    ///
    /// // Error case - type annotation needed when only using the error
    /// let err = HashMap::<i32, i32, RandomState>::try_from_iter([(1, 2), (1, 3)]).expect_err("should fail on collision");
    /// assert_eq!(err.error.item, (1, 3), "colliding item should be (1, 3)");
    /// ```    
    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error>;
}

/// Extends [Iterator] with a failable collect method.
///
/// This trait allows an iterator to return any collection that can be created via
/// [`TryFromIterator`], similar to [`Iterator::collect`] and [`FromIterator::from_iter`],
/// but with the ability to return an implementation-specific error if the creation of the container
/// fails some invariant.
#[sealed::sealed]
pub trait TryCollectEx: Iterator {
    /// Tries to collect the iterator into a container, returning an error if constructing the
    /// container fails.
    ///
    /// Exact behavior of this method depends on the container implementation, but generally it
    /// should be expected to short-circuit on the first error. On success, this method should
    /// behave similarly to [`Iterator::collect`], except returning a [`Result`].
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
    /// use collect_failable::TryCollectEx;
    /// use std::collections::HashMap;
    ///
    /// let data = [(1, 2), (1, 3)];
    /// let result = data.into_iter().try_collect_ex::<HashMap<_, _>>().expect_err("should fail on collision");
    /// assert_eq!(result.error.item, (1, 3), "colliding item should be (1, 3)");
    /// ```
    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self>,
        Self: Sized;
}

/// Implementation of [`TryCollectEx`] for all [`Iterator`].
#[sealed::sealed]
impl<I, T> TryCollectEx for I
where
    I: Iterator<Item = T>,
{
    fn try_collect_ex<C>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self>,
    {
        C::try_from_iter(self)
    }
}
