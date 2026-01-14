/// Trait for error types that may contain a rejected item.
///
/// This trait allows [`CollectionError`](super::CollectionError) to extract rejected items from
/// errors for reconstruction via [`IntoIterator::into_iter`].
///
/// # Examples
///
/// ```
/// use collect_failable::errors::{Collision, ErrorItemProvider};
///
/// let error = Collision::new(42);
/// assert_eq!(error.item(), Some(&42));
/// assert_eq!(error.into_item(), Some(42));
/// ```
pub trait ErrorItemProvider {
    /// The type of the item that may be contained in this error.
    type Item;

    /// Consumes the error and returns the rejected item, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_failable::errors::{Collision, ErrorItemProvider};
    ///
    /// let error = Collision::new(42);
    /// assert_eq!(error.into_item(), Some(42));
    /// ```
    #[must_use]
    fn into_item(self) -> Option<Self::Item>;

    /// Returns a reference to the rejected item, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use collect_failable::errors::{Collision, ErrorItemProvider};
    ///
    /// let error = Collision::new(42);
    /// assert_eq!(error.item(), Some(&42));
    /// ```
    #[must_use]
    fn item(&self) -> Option<&Self::Item>;
}
