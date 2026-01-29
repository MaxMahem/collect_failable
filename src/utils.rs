/// Ensures an iterator is empty.
pub trait EnsureEmpty: Iterator + Sized {
    /// Checks if the iterator is empty.
    ///
    /// # Errors
    ///
    /// Returns [`NotEmpty`] if the iterator is not empty.
    fn ensure_empty(mut self) -> Result<(), NotEmpty<Self>> {
        self.next().map_or_else(|| Ok(()), |item| Err(NotEmpty { iter: self, item }))
    }
}

impl<I: Iterator> EnsureEmpty for I {}

/// An error indicating that an iterator is not empty.
#[derive(Debug, thiserror::Error)]
#[error("iterator is not empty")]
pub struct NotEmpty<I: Iterator> {
    // The iterator with the first item removed
    pub iter: I,
    // The first item yielded by the iterator
    pub item: I::Item,
}
