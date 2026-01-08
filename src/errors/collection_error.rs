use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use core::iter::Chain;
use core::ops::Deref;

use alloc::boxed::Box;

use display_as_debug::option::OptionDebugExt;
use size_hinter::SizeHint;
use tap::Pipe;

use super::CapacityMismatch;

#[cfg(doc)]
use crate::errors::MismatchKind;

/// An error that occurs when an collecting an iterator fails during it's collection.
///
/// Note this type is *read-only*. The fields are accessible via a hidden [`Deref`](std::ops::Deref).
/// implementation into a hidden `CollectionErrorData` type, with identical fields. If necessary,
/// you can consume an instance of this type via [`CollectionError::into_data`] to get owned data.
///
/// # Type Parameters
///
/// - `I`: The type of the iterator.
/// - `C`: The type of the collection.
/// - `E`: The type of the nested error.
#[subdef::subdef]
//#[derive(derive_more::Deref)]
pub struct CollectionError<I: Iterator, C, E> {
    #[cfg(doc)]
    /// The iterator that was partially iterated
    pub iterator: I,
    #[cfg(doc)]
    /// The values that were collected
    pub collected: C,
    #[cfg(doc)]
    /// An optional item that was rejected (consumed but couldn't be added)
    pub rejected: Option<I::Item>,
    #[cfg(doc)]
    /// The error that occurred
    pub error: E,

    #[cfg(not(doc))]
    //#[deref(forward)]
    data: [Box<CollectionErrorData<I, C, E>>; {
        /// The internal data of a [`CollectionError`].
        #[doc(hidden)]
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
}

impl<I: Iterator, C, E> CollectionError<I, C, E> {
    /// Creates a new [`CollectionError`] from an `iterator`, `collected` values, optional `rejected` item, and a nested `error`.
    pub fn new(iterator: I, collected: C, rejected: Option<I::Item>, error: E) -> Self {
        CollectionErrorData { iterator, collected, rejected, error }.pipe(Box::new).pipe(|data| Self { data })
    }

    /// Consumes the error, returning a `CollectionErrorData` containing the [`CollectionError::iterator`],
    /// [`CollectionError::collected`] values, the optional [`CollectionError::rejected`] item, and nested
    /// [`CollectionError::error`].
    #[must_use]
    pub fn into_data(self) -> CollectionErrorData<I, C, E> {
        *self.data
    }
}

#[doc(hidden)]
impl<I: Iterator, C, E> Deref for CollectionError<I, C, E> {
    type Target = CollectionErrorData<I, C, E>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I: Iterator, C> CollectionError<I, C, CapacityMismatch> {
    /// Creates a new [`CollectionError`] with a [`CapacityMismatch`] error of type [`MismatchKind::Bounds`].
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
    /// Panics if the iterator's size hint is invalid.
    #[must_use]
    #[inline]
    pub fn bounds(iterator: I, capacity: SizeHint) -> Self
    where
        C: Default,
    {
        let hint = iterator.size_hint().try_into().unwrap();
        Self::new(iterator, C::default(), None, CapacityMismatch::bounds(capacity, hint))
    }

    /// Creates a new [`CollectionError`] with a [`CapacityMismatch`] error of type [`MismatchKind::Overflow`].
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
    pub fn overflow(iterator: I, collected: C, rejected: I::Item, capacity: SizeHint) -> Self {
        Self::new(iterator, collected, Some(rejected), CapacityMismatch::overflow(capacity))
    }

    /// Creates a new [`CollectionError`] with a [`CapacityMismatch`] error of type [`MismatchKind::Underflow`].
    ///
    /// This is a convenience method for creating errors when the iterator produced fewer items
    /// than the minimum allowed capacity during actual collection.
    ///
    /// # Arguments
    ///
    /// * `iterator` - The remaining iterator after underflow occurred
    /// * `collected` - The values that were collected before underflow
    /// * `capacity` - The allowed capacity range for the collection
    #[must_use]
    #[inline]
    pub fn underflow(iterator: I, collected: C, capacity: SizeHint) -> Self
    where
        for<'a> &'a C: IntoIterator<IntoIter: ExactSizeIterator>,
    {
        let count = (&collected).into_iter().len();
        Self::new(iterator, collected, None, CapacityMismatch::underflow(capacity, count))
    }
}

impl<I: Iterator, C: IntoIterator<Item = I::Item>, E> IntoIterator for CollectionError<I, C, E> {
    type Item = I::Item;
    type IntoIter = Chain<Chain<core::option::IntoIter<I::Item>, C::IntoIter>, I>;

    /// Consumes the error, and reconstructs the iterator it was created from. This will include
    /// the `rejected` item, `collected` values, and the remaining `iterator`, in that order.
    fn into_iter(self) -> Self::IntoIter {
        self.data.rejected.into_iter().chain(self.data.collected).chain(self.data.iterator)
    }
}

impl<I: Iterator, C, E: Display> Display for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<I: Iterator, C, E: Error + Debug + 'static> Error for CollectionError<I, C, E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl<I: Iterator, C, E: Debug> Debug for CollectionError<I, C, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PartialIterErr")
            .field("collected", &core::any::type_name::<C>())
            .field("rejected", &self.rejected.debug_opaque())
            .field("error", &self.error)
            .field("iterator", &core::any::type_name::<I>())
            .finish()
    }
}
