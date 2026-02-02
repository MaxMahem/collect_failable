/// A type suitable for use as an index in an array of size `N`.
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::Into,
    derive_more::From,
    derive_more::Deref,
    derive_more::AsRef,
)]
pub struct ArrayIndex<const N: usize>(usize);

impl<const N: usize> ArrayIndex<N> {
    /// Creates a new `ArrayIndex` starting at 0.
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Returns the current index if it is within bounds, then increments the internal counter.
    ///
    /// If the index is already at or beyond `N`, returns `None`.
    pub fn try_post_inc(&mut self) -> Option<usize> {
        (self.0 < N).then(|| self.0.post_inc())
    }
}

/// A trait for types that can be post-incremented.
#[sealed::sealed]
pub trait PostInc: Copy + core::ops::AddAssign<usize> {
    /// Post-increments the value.
    #[must_use]
    fn post_inc(&mut self) -> Self {
        let old = *self;
        *self += 1;
        old
    }
}

#[sealed::sealed]
impl PostInc for usize {}
