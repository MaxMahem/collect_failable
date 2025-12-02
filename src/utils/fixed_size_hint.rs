pub type SizeHint = (usize, Option<usize>);

/// An iterator adaptor that overrides the size hint of the underlying iterator.
///
/// This is useful for easily hiding the size hint of an iterator in tests.
#[allow(dead_code)]
#[derive(Debug)]
pub struct FixedSizeHint<I: Iterator> {
    /// The underlying iterator.
    pub iterator: I,
    /// The size hint to return.
    pub size_hint: SizeHint,
}

/// A universally applicable size hint that conveys no information.
pub const UNIVERSAL_SIZE_HINT: SizeHint = (0, None);

impl<I: Iterator> FixedSizeHint<I> {
    /// Creates a new iterator with the size hint hidden.
    pub fn hide_size(iterator: impl IntoIterator<IntoIter = I>) -> Self {
        Self::fixed_size(iterator, UNIVERSAL_SIZE_HINT)
    }

    /// Creates a new iterator with the given fixed size hint.
    pub fn fixed_size(iterator: impl IntoIterator<IntoIter = I>, size_hint: SizeHint) -> Self {
        Self { iterator: iterator.into_iter(), size_hint }
    }
}

impl<I: Iterator> Iterator for FixedSizeHint<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }
}

/// Extension trait for [`Iterator`] to fluently override or hide size hints.
pub trait FixedSizeHintEx: Iterator + Sized {
    /// Creates a new iterator with the size hint hidden.
    fn hide_size(self) -> FixedSizeHint<Self> {
        self.fixed_size(UNIVERSAL_SIZE_HINT)
    }

    /// Creates a new iterator with the given fixed size hint.
    fn fixed_size(self, size_hint: SizeHint) -> FixedSizeHint<Self> {
        FixedSizeHint::fixed_size(self, size_hint)
    }
}

impl<T: Iterator> FixedSizeHintEx for T {}
