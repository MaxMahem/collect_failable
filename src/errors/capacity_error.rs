use crate::SizeHint;

use super::ErrorItemProvider;

/// An error indicating that a collection operation could not proceed because the number of items
/// produced by an [`Iterator`] violated the collection’s declared capacity.
///
/// This error distinguishes three failure modes, identified by [`CapacityError::kind`].
///
/// - [`CapacityErrorKind::Bounds`] — the iterator’s reported size bounds are incompatible with
///   the required capacity, making the operation impossible to satisfy even before iteration begins.
///
/// - [`CapacityErrorKind::Underflow`] — the iterator produced fewer items than the minimum required
///   by the capacity.
///
/// - [`CapacityErrorKind::Overflow`] — the iterator produced more items than the maximum allowed by
///   the capacity. In this case, the first rejected item is preserved so that it may be recovered
///   by callers via [`ErrorItemProvider`].
///
/// # Type Parameters
///
/// - `T`: The type of the item produced by the iterator. It is only present in the `Overflow` case,
///   where the rejected item must be returned to the caller.
#[subdef::subdef]
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Collected items out of bounds ({capacity:?}): {kind}")]
pub struct CapacityError<T> {
    /// The capacity constraint that was violated.
    ///
    /// The exact meaning of this field depends on the operation context:
    ///
    /// - **Collection operations** ([`TryFromIterator::try_from_iter`](crate::TryFromIterator::try_from_iter))
    ///   — represents the total capacity of the collection being constructed.
    ///
    /// - **Extension operations** ([`TryExtend::try_extend`](crate::TryExtend::try_extend),
    ///  [`TryExtendSafe::try_extend_safe`](crate::TryExtendSafe::try_extend_safe))
    ///   — represents the remaining capacity available in the target collection after the failure.
    ///
    pub capacity: SizeHint,
    /// The specific kind of capacity mismatch that occurred.
    pub kind: [CapacityErrorKind<T>; {
        /// Describes the specific type of capacity mismatch.
        #[derive(Debug, PartialEq, Eq, derive_more::Display)]
        pub enum CapacityErrorKind<T> {
            /// The iterator's reported size bounds ([`Iterator::size_hint`]) are incompatible with
            /// the required capacity.
            #[display("Item count bounds ({hint:?}) cannot satisfy capacity")]
            Bounds {
                /// The iterator's reported size bounds.
                hint: SizeHint,
            },
            /// The iterator produced fewer items than the minimum required capacity.
            #[display("Fewer ({count}) items than necessary")]
            Underflow {
                /// The actual number of items produced.
                count: usize,
            },
            /// The iterator produced more items than the maximum allowed capacity.
            #[display("Item count exceeds capacity")]
            Overflow {
                /// The item that exceeded capacity.
                rejected: T,
            },
        }
    }],
}

impl<T> CapacityError<T> {
    /// Creates a new [`CapacityError`] indicating that the bounds provided by [`Iterator::size_hint`]
    /// were incompatible with the required `capacity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let err = CapacityError::<i32>::bounds(SizeHint::exact(5), SizeHint::exact(2));
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Bounds { hint: SizeHint::exact(2) });
    /// ```
    #[must_use]
    pub const fn bounds(capacity: SizeHint, hint: SizeHint) -> Self {
        Self { capacity, kind: CapacityErrorKind::Bounds { hint } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator exceeded the maximum capacity.
    ///
    /// The number of excess items produced is implied to be at least one, but may be greater.
    ///
    /// Use this when you know the exact capacity constraint. For extension operations that fill
    /// a collection to capacity, consider [`CapacityError::overflowed`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let err = CapacityError::overflow(SizeHint::exact(5), 2);
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { rejected: 2 });
    /// ```
    #[must_use]
    pub const fn overflow(capacity: SizeHint, rejected: T) -> Self {
        Self { capacity, kind: CapacityErrorKind::Overflow { rejected } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator overflowed the maximum capacity,
    /// and the target collection is left in a full state.
    ///
    /// The number of excess items produced is implied to be at least one, but may be greater.
    /// Since the collection is full, the capacity is always [`SizeHint::ZERO`].
    ///
    /// # When to Use
    ///
    /// This constructor is intended for **extension operations** that leave the target collection
    /// in a full state, such as [`TryExtend::try_extend`](crate::TryExtend::try_extend). For
    /// overflow failures on operations that do not leave the target collection in a full state,
    /// use [`CapacityError::overflow`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let err = CapacityError::overflowed(2);
    /// assert_eq!(err.capacity, SizeHint::ZERO);
    /// assert_eq!(err.kind, CapacityErrorKind::Overflow { rejected: 2 });
    /// ```
    #[must_use]
    pub const fn overflowed(rejected: T) -> Self {
        Self { capacity: SizeHint::ZERO, kind: CapacityErrorKind::Overflow { rejected } }
    }

    /// Creates a new [`CapacityError`] indicating that the iterator produced fewer items than the
    /// minimum required by `capacity`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use collect_failable::errors::{CapacityError, CapacityErrorKind};
    /// use collect_failable::SizeHint;
    ///
    /// let err = CapacityError::<i32>::underflow(SizeHint::exact(5), 2);
    /// assert_eq!(err.capacity, SizeHint::exact(5));
    /// assert_eq!(err.kind, CapacityErrorKind::Underflow { count: 2 });
    /// ```
    #[must_use]
    pub const fn underflow(capacity: SizeHint, count: usize) -> Self {
        Self { capacity, kind: CapacityErrorKind::Underflow { count } }
    }
}

impl<T> ErrorItemProvider for CapacityError<T> {
    type Item = T;

    fn into_item(self) -> Option<Self::Item> {
        match self.kind {
            CapacityErrorKind::Overflow { rejected } => Some(rejected),
            _ => None,
        }
    }

    fn item(&self) -> Option<&Self::Item> {
        match &self.kind {
            CapacityErrorKind::Overflow { rejected } => Some(rejected),
            _ => None,
        }
    }
}
