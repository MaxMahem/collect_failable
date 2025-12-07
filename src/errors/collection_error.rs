use tap::{Conv, Pipe};
use crate::utils::DebugOption;

/// An error that occurs when an operation on an iterator fails mid way through it's iteration.
/// 
/// For example, when using `try_collect` on an iterator that fails mid way through it's iteration,
/// this error can be used to return the values collected so far, the partially iterated iter, and
/// a nested error.
#[derive(derive_more::Deref)]
#[deref(forward)]
pub struct CollectionError<T, I, C, E>(Box<ReadOnlyCollectionError<T, I, C, E>>) where 
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>;

impl<T, I, C, E> CollectionError<T, I, C, E> where 
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T> 
{
    /// Creates a new [`PartialIterErr`] from an `iterator`, `collected` values, optional `rejected` item, and a nested `error`.
    pub fn new(iterator: I, collected: C, rejected: Option<T>, error: E) -> Self {
        ReadOnlyCollectionError { iterator, collected, rejected, error }.pipe(Box::new).pipe(CollectionError)
    }

    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_err(self) -> E {
        self.0.error
    }

    /// Consumes the error, returning a [`ReadOnlyPartialIterErr`] containing the `iterator`, 
    /// `collected` values, the optional `rejected` item, and nested `error`.
    #[must_use]
    pub fn into_parts(self) -> ReadOnlyCollectionError<T, I, C, E> {
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

impl<T, I, C, E> IntoIterator for CollectionError<T, I, C, E> 
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T> 
{
    type Item = T;
    type IntoIter = std::iter::Chain<std::iter::Chain<std::option::IntoIter<T>, C::IntoIter>, I>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.rejected.into_iter().chain(self.0.collected).chain(self.0.iterator)
    }
}

impl<T, I, C, E: std::fmt::Display> std::fmt::Display for CollectionError<T, I, C, E>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl<T, I, C, E: std::error::Error> std::error::Error for CollectionError<T, I, C, E> 
where 
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T> + std::fmt::Debug,
    E: std::fmt::Debug + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl<T, I, C, E> std::fmt::Debug for CollectionError<T, I, C, E>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartialIterErr")
            .field("collected", &std::any::type_name::<C>())
            .field("rejected", &DebugOption(&self.rejected))
            .field("error", &self.error)
            .field("iterator", &std::any::type_name::<I>())
            .finish()
    }
}

/// A read only version of [`CollectionError`].
pub struct ReadOnlyCollectionError<T, I, C, E> 
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    /// The iterator that was partially iterated
    pub iterator: I,
    /// The values that were collected
    pub collected: C,
    /// The item that was rejected (consumed but couldn't be added)
    pub rejected: Option<T>,
    /// The error that occurred
    pub error: E,
}
