use arrayvec::ArrayVec;
use fluent_result::bool::Then;

use crate::{ExceedsCapacity, CollectionError, TryExtend, TryExtendSafe, TryFromIterator};

/// Tries to create an [`ArrayVec`] from an iterator.
///
/// This implementation will return an error if the iterator produces more items than the [`ArrayVec`]'s capacity.
///
/// # Examples
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_from_iter_arrayvec_example, [])]
/// ```
impl<T, I, const N: usize> TryFromIterator<T, I> for ArrayVec<T, N> 
where
    I: IntoIterator<Item = T>
{
    type Error = CollectionError<T, I::IntoIter, ArrayVec<T, N>, ExceedsCapacity>;

    fn try_from_iter(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();
        
        iter.try_fold(ArrayVec::new(), |mut array, item| {
            match array.try_push(item) {
                Ok(()) => Ok(array),
                Err(capacity_err) => Err((array, capacity_err.element())),
            }
        })
        .map_err(|(array, rejected_item)| {
            CollectionError::new(iter, array, Some(rejected_item), ExceedsCapacity::new(N, N + 1))
        })
    }
}

/// Extends an [`ArrayVec`] with an iterator, failing if the iterator produces more items than the [`ArrayVec`]'s
/// remaining capacity.
impl<T, const N: usize> TryExtend<T> for ArrayVec<T, N> {
    type Error = ExceedsCapacity;

    /// Appends an iterator to the [`ArrayVec`], failing if the iterator produces more items than the [`ArrayVec`]'s
    /// remaining capacity.
    ///
    /// This method provides a basic error guarantee. If the method returns an error, the [`ArrayVec`] is valid, but may
    /// be modified.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_extend_arrayvec_example, [])]
    /// ```
    fn try_extend<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let size_guess = iter.size_hint().0;
        (size_guess > self.remaining_capacity()).then_err(ExceedsCapacity::new(N, self.len() + size_guess))?;

        iter.try_for_each(|item| self.try_push(item).map_err(|_| ExceedsCapacity::new(N, N + 1)))
    }
}

/// Extends an [`ArrayVec`] with strong error guarantee.
impl<T, const N: usize> TryExtendSafe<T> for ArrayVec<T, N> {
    /// Appends an iterator to the [`ArrayVec`], failing if the iterator produces more items than the [`ArrayVec`]'s
    /// remaining capacity.
    ///
    /// This method provides a strong error guarantee. If the method returns an error, the [`ArrayVec`] is not modified.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/arrayvec.rs", try_extend_safe_arrayvec_example, [])]
    /// ```
    fn try_extend_safe<I>(&mut self, iter: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();

        let size_guess = iter.size_hint().0;
        (size_guess > self.remaining_capacity()).then_err(ExceedsCapacity::new(N, self.len() + size_guess))?;

        let len = self.len();

        iter.try_for_each(|item| self.try_push(item).map_err(|_| ExceedsCapacity::new(N, N + 1))).inspect_err(|_| {
            self.truncate(len);
        })
    }
}
