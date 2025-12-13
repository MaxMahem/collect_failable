use display_as_debug::option::OpaqueOptionDbg;
use tap::{Conv, Pipe};

/// An error that occurs when an collecting an iterator fails during it's collection.
#[subdef::subdef]
#[derive(derive_more::Deref)]
#[deref(forward)]
pub struct CollectionError<T, I, C, E>(
    [Box<CollectionErrorData<T, I, C, E>>; {
        /// The internal data of a [`CollectionError`].
        pub struct CollectionErrorData<T, I, C, E>
        where
            I: Iterator<Item = T>,
            C: IntoIterator<Item = T>,
        {
            /// The iterator that was partially iterated
            pub iterator: I,
            /// The values that were collected
            pub collected: C,
            /// An optional item that was rejected (consumed but couldn't be added)
            pub rejected: Option<T>,
            /// The error that occurred
            pub error: E,
        }
    }],
)
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>;

impl<T, I, C, E> CollectionError<T, I, C, E>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
{
    /// Creates a new [`CollectionError`] from an `iterator`, `collected` values, optional `rejected` item, and a nested `error`.
    pub fn new(iterator: I, collected: C, rejected: Option<T>, error: E) -> Self {
        CollectionErrorData { iterator, collected, rejected, error }.pipe(Box::new).pipe(CollectionError)
    }

    /// Consumes the error, returning the nested error.
    #[must_use]
    pub fn into_err(self) -> E {
        self.0.error
    }

    /// Consumes the error, returning a [`CollectionErrorData`] containing the `iterator`,
    /// `collected` values, the optional `rejected` item, and nested `error`.
    #[must_use]
    pub fn into_parts(self) -> CollectionErrorData<T, I, C, E> {
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
    C: IntoIterator<Item = T>,
{
    type Item = T;
    type IntoIter = std::iter::Chain<std::iter::Chain<std::option::IntoIter<T>, C::IntoIter>, I>;

    /// Consumes the error, and reconstructs the iterator it was created from. This will include
    /// the `rejected` item, `collected` values, and the remaining `iterator`, in that order.
    fn into_iter(self) -> Self::IntoIter {
        self.0.rejected.into_iter().chain(self.0.collected).chain(self.0.iterator)
    }
}

impl<T, I, C, E: std::fmt::Display> std::fmt::Display for CollectionError<T, I, C, E>
where
    I: Iterator<Item = T>,
    C: IntoIterator<Item = T>,
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
            .field("rejected", &OpaqueOptionDbg(&self.rejected))
            .field("error", &self.error)
            .field("iterator", &std::any::type_name::<I>())
            .finish()
    }
}
