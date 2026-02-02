use fluent_result::into::IntoResult;

/// Ensures an [`Iterator`] is empty.
#[sealed::sealed]
pub trait EnsureEmpty: Iterator + Sized {
    /// Checks if the [`Iterator`] is empty.
    ///
    /// # Errors
    ///
    /// Returns [`NotEmpty`] if the [`Iterator`] is not empty.
    fn ensure_empty(mut self) -> Result<(), NotEmpty<Self>> {
        self.next().map_or_else(|| Ok(()), |item| NotEmpty { iter: self, item }.into_err())
    }
}

#[sealed::sealed]
impl<I: Iterator> EnsureEmpty for I {}

/// An error indicating that an [`Iterator`] is not empty.
#[derive(Debug, thiserror::Error)]
#[error("iterator is not empty")]
pub struct NotEmpty<I: Iterator> {
    // The iterator with the first item removed
    pub iter: I,
    // The first item yielded by the iterator
    pub item: I::Item,
}
