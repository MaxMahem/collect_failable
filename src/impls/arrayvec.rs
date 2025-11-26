use arrayvec::ArrayVec;
use fluent_result::ThenErr;

use crate::{ExceedsCapacity, FoldMut, TryExtend, TryFromIterator};

/// Tries to create an `ArrayVec` from an iterator.
///
/// This implementation will return an error if the iterator produces more items than the `ArrayVec`'s capacity.
///
/// # Examples
///
/// ```
/// use arrayvec::ArrayVec;
/// use collect_failable::TryFromIterator;
///
/// let data = [1, 2, 3];
/// let array: ArrayVec<i32, 4> = ArrayVec::try_from_iter(data).unwrap();
/// assert_eq!(array.as_slice(), &[1, 2, 3]);
///
/// let data = [1, 2, 3, 4, 5];
/// let res: Result<ArrayVec<i32, 4>, _> = ArrayVec::try_from_iter(data);
/// assert!(res.is_err());
/// assert_eq!(res.unwrap_err().to_string(), "Too many items to fit the array, Capacity: 4, necessary: 5");
/// ```
impl<T, const N: usize> TryFromIterator<T> for ArrayVec<T, N> {
    type Error = ExceedsCapacity;

    fn try_from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Result<Self, Self::Error> {
        let mut iter = into_iter.into_iter();
        let size_guess = iter.size_hint().0;

        (size_guess > N).then_err(ExceedsCapacity::new(N, size_guess))?;

        iter.try_fold_mut(ArrayVec::new(), |array, item| {
            array.try_push(item).map_err(|_| ExceedsCapacity::new(N, N + 1))
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
    /// This method provides a strong error guarantee. If the method returns an error, the [`ArrayVec`] is not modified.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayvec::ArrayVec;
    /// use collect_failable::{TryExtend, ExceedsCapacity};
    ///
    /// let mut array: ArrayVec<i32, 4> = ArrayVec::new();
    /// array.push(1);
    /// array.push(2);
    ///
    /// array.try_extend_safe([3]).expect("Should be ok");
    /// assert_eq!(*array, [1, 2, 3]);
    ///
    /// let err = array.try_extend_safe([4, 5]).expect_err("Should be err");
    /// assert_eq!(err, ExceedsCapacity {capacity: 4, necessary: 5 });
    /// assert_eq!(*array, [1, 2, 3]); // Unchanged
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

    /// Appends an iterator to the [`ArrayVec`], failing if the iterator produces more items than the [`ArrayVec`]'s
    /// remaining capacity.
    ///
    /// This method provides a basic error guarantee. If the method returns an error, the [`ArrayVec`] is valid, but may
    /// be modified.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayvec::ArrayVec;
    /// use collect_failable::{TryExtend, ExceedsCapacity};
    ///
    /// let mut array: ArrayVec<i32, 4> = ArrayVec::new();
    /// array.push(1);
    /// array.push(2);
    ///
    /// array.try_extend([3]).expect("Should be ok");
    /// assert_eq!(*array, [1, 2, 3]);
    ///
    /// let err = array.try_extend([4, 5]).expect_err("Should be err");
    /// assert_eq!(err, ExceedsCapacity {capacity: 4, necessary: 5 });
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
