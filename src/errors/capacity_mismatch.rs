use std::ops::{Range, RangeInclusive};

/// An error indicating that the a collection was not created because the capacity was violated.
#[subdef::subdef(derive(Debug, PartialEq, Eq))]
#[derive(thiserror::Error)]
#[error("Collected items outside allowed range ({min}..{max}): {kind}", min = self.capacity.start, max = self.capacity.end)]
#[readonly::make]
pub struct CapacityMismatch {
    /// The allowed capacity range for the collection.
    pub capacity: Range<usize>,
    /// The specific kind of capacity mismatch that occurred.
    pub kind: [_; {
        /// Describes the specific type of capacity mismatch.
        #[derive(derive_more::Display)]
        pub enum MismatchKind {
            /// The item collections bounds cannot fit within the allowed capacity.
            #[display("Item count bounds ({min}..={max:?}) cannot satisfy capacity")]
            Bounds {
                /// The minimum bound.
                min: usize,
                /// An optional maximum bound
                max: Option<usize>,
            },
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
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the hint does not indicate a bounds error.
    #[must_use]
    pub const fn bounds(capacity: RangeInclusive<usize>, hint: (usize, Option<usize>)) -> Self {
        // Bounds error occurs when:
        // 1. The hint's minimum is greater than capacity max (definitely too many items), OR
        // 2. The hint's maximum is less than capacity min (definitely too few items)
        debug_assert!(
            hint.0 > *capacity.end() || (hint.1.is_some() && hint.1.unwrap() < *capacity.start()),
            "hint must be outside capacity range"
        );
        Self {
            capacity: *capacity.start()..capacity.end().saturating_add(1),
            kind: MismatchKind::Bounds { min: hint.0, max: hint.1 },
        }
    }

    /// Creates a new [`CapacityMismatch`] indicating that the iterator exceeded the expected capacity.
    #[must_use]
    pub const fn overflow(capacity: RangeInclusive<usize>) -> Self {
        Self { capacity: *capacity.start()..capacity.end().saturating_add(1), kind: MismatchKind::Overflow }
    }

    /// Creates a new [`CapacityMismatch`] indicating that the iterator did not produce the expected number of items.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the count is not less than the capacity minimum.
    #[must_use]
    pub const fn underflow(capacity: RangeInclusive<usize>, count: usize) -> Self {
        debug_assert!(count < *capacity.start(), "count must be less than capacity minimum");
        Self { capacity: *capacity.start()..capacity.end().saturating_add(1), kind: MismatchKind::Underflow { count } }
    }
}
