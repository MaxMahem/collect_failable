use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Chain;
use std::ops::RangeInclusive;

use display_as_debug::option::OpaqueOptionDbg;
use tap::{Conv, Pipe};

use crate::CapacityMismatch;

/// An error that occurs when an collecting an iterator fails during it's collection.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator.
/// - `C`: The type of the collection.
/// - `E`: The type of the nested error.
#[subdef::subdef]
#[derive(derive_more::Deref)]
#[deref(forward)]
pub struct CollectionError<I: Iterator, C, E>(
    [Box<CollectionErrorData<I, C, E>>; {
        /// The internal data of a [`CollectionError`].
        pub struct CollectionErrorData<I: Iterator, C, E> {
            /// The iterator that was partially iterated
            pub iterator: I,
            /// The values that were collected
            pub collected: C,
            /// An optional item that was rejected (consumed but couldn't be added)
            pub rejected: Option<I::Item>,
            /// The error that occurred
            pub error: E,
        }
    }],
);

impl<I: Iterator, C, E> CollectionError<I, C, E> {
    /// Creates a new [`CollectionError`] from an `iterator`, `collected` values, optional `rejected` item, and a nested `error`.
    pub fn new(iterator: I, collected: C, rejected: Option<I::Item>, error: E) -> Self {
        CollectionErrorData { iterator, collected, rejected, error }.pipe(Box::new).pipe(CollectionError)
    }

    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_error(self) -> E {
        self.0.error
    }

    /// Consumes the error, returning a [`CollectionErrorData`] containing the `iterator`,
    /// `collected` values, the optional `rejected` item, and nested `error`.
    #[must_use]
    pub fn into_data(self) -> CollectionErrorData<I, C, E> {
        *self.0
    }

    /// Returns the number of elements in the `iterator`, `collected` values, and `rejected` item.
    #[must_use]
    pub fn len(&self) -> usize
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        (&self.0.collected).into_iter().len() + self.0.iterator.len() + self.0.rejected.is_some().conv::<usize>()
    }

    /// Returns `true` if the iterator and collected values are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        I: ExactSizeIterator,
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        self.len() == 0
    }
}

impl<I: Iterator, C> CollectionError<I, C, CapacityMismatch> {
    /// Creates a new [`CollectionError`] with a [`CapacityMismatch::Bounds`](crate::CapacityMismatch) error.
    ///
    /// This is a convenience method for creating errors when a pre-check of an iterator's size hint
    /// indicates that it cannot fit within the specified capacity. A [`Default`]ed collection is used for
    /// the `collected` values, and [`None`] is used for the `rejected` item, since no items should be collected
    /// yet.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The iterator that failed the bounds check
    /// * `capacity` - The allowed capacity range for the collection
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the hint does not indicate a bounds error.
    #[must_use]
    #[inline]
    pub(crate) fn bounds(iterator: I, capacity: RangeInclusive<usize>) -> Self
    where
        C: Default,
    {
        let hint = iterator.size_hint();
        Self::new(iterator, C::default(), None, CapacityMismatch::bounds(capacity, hint))
    }

    /// Creates a new [`CollectionError`] with a [`CapacityMismatch::Overflow`](crate::CapacityMismatch) error.
    ///
    /// This is a convenience method for creating errors when the iterator produced more items
    /// than the maximum allowed capacity during actual collection.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after overflow occurred
    /// * `collected` - The values that were collected before overflow
    /// * `rejected` - The item that was consumed but couldn't be added to the collection
    /// * `capacity` - The allowed capacity range for the collection
    #[must_use]
    #[inline]
    pub(crate) fn overflow(iterator: I, collected: C, rejected: I::Item, capacity: RangeInclusive<usize>) -> Self {
        Self::new(iterator, collected, Some(rejected), CapacityMismatch::overflow(capacity))
    }
}

impl<I: Iterator, C: IntoIterator<Item = I::Item>, E> IntoIterator for CollectionError<I, C, E> {
    type Item = I::Item;
    type IntoIter = Chain<Chain<std::option::IntoIter<I::Item>, C::IntoIter>, I>;

    /// Consumes the error, and reconstructs the iterator it was created from. This will include
    /// the `rejected` item, `collected` values, and the remaining `iterator`, in that order.
    fn into_iter(self) -> Self::IntoIter {
        self.0.rejected.into_iter().chain(self.0.collected).chain(self.0.iterator)
    }
}

impl<I: Iterator, C, E: Display> Display for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<I: Iterator, C, E: Error + Debug + 'static> Error for CollectionError<I, C, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I: Iterator, C, E: Debug> Debug for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialIterErr")
            .field("collected", &std::any::type_name::<C>())
            .field("rejected", &OpaqueOptionDbg(&self.rejected))
            .field("error", &self.error)
            .field("iterator", &std::any::type_name::<I>())
            .finish()
    }
}
