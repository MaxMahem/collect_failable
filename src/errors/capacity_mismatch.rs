use size_hinter::SizeHint;

/// An error indicating that the a collection was not created because the capacity was violated.
#[subdef::subdef(derive(Debug, PartialEq, Eq))]
#[derive(thiserror::Error)]
#[error("Collected items out of bounds ({capacity:?}): {kind}")]
pub struct CapacityMismatch {
    /// The allowed capacity for the collection.
    pub capacity: SizeHint,
    /// The specific kind of capacity mismatch that occurred.
    pub kind: [_; {
        /// Describes the specific type of capacity mismatch.
        #[derive(derive_more::Display)]
        pub enum MismatchKind {
            /// The item collections bounds cannot fit within the allowed capacity.
            #[display("Item count bounds ({_0:?}) cannot satisfy capacity")]
            Bounds(SizeHint),
            /// The iterator produced fewer items than the minimum required capacity.
            #[display("Fewer ({count}) items than necessary")]
            Underflow {
                /// The actual number of items produced.
                count: usize,
            },
            /// The iterator produced more items than the maximum allowed capacity.
            #[display("Item count exceeds capacity")]
            Overflow,
        }
    }],
}

impl CapacityMismatch {
    /// Creates a new [`CapacityMismatch`] indicating that the bounds provided by [`Iterator::size_hint`] were out of bounds.
    #[must_use]
    pub const fn bounds(capacity: SizeHint, hint: SizeHint) -> Self {
        Self { capacity, kind: MismatchKind::Bounds(hint) }
    }

    /// Creates a new [`CapacityMismatch`] indicating that the iterator exceeded the expected capacity.
    #[must_use]
    pub const fn overflow(capacity: SizeHint) -> Self {
        Self { capacity, kind: MismatchKind::Overflow }
    }

    /// Creates a new [`CapacityMismatch`] indicating that the iterator did not produce the expected number of items.
    #[must_use]
    pub const fn underflow(capacity: SizeHint, count: usize) -> Self {
        Self { capacity, kind: MismatchKind::Underflow { count } }
    }
}
