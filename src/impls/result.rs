use fluent_result::bool::Then;

use crate::TryFromIterator;

/// Iterator adaptor that extracts [`Ok`] values from a [`Result`] [`Iterator`],
/// storing the first encountered [`Err`] for later retrieval.
#[derive(Debug)]
struct ExtractErr<I, E> {
    iter: I,
    error: Option<E>,
}

impl<I, E> From<I> for ExtractErr<I, E> {
    fn from(iter: I) -> Self {
        Self { iter, error: None }
    }
}

/// Implements Iterator for `ExtractErr`, yielding Ok values and capturing the first Err.
///
/// Once an Err is encountered, the iterator terminates and stores the error.
impl<I, T, E> Iterator for ExtractErr<I, E>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.error.is_some().then_none()?;

        match self.iter.next() {
            Some(Ok(v)) => Some(v),
            #[rustfmt::skip]
            Some(Err(e)) => { self.error = Some(e); None }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.error.is_some() {
            true => (0, Some(0)),
            false => (0, self.iter.size_hint().1),
        }
    }
}

/// Converts an iterator of [`Result<T, E>`] into a [`Result<C, E>`], where `C` implements [`TryFromIterator<T>`].
///
/// That is, given a iterator that yields [`Result<T, E>`], this implementation will collect all [`Ok`] values
/// into a container `C` that implements [`TryFromIterator<T>`], short-circuiting on the first [`Err`] encountered.
/// If an [`Err`] is found, it is returned immediately. If all values are [`Ok`], but the inner collection fails to
/// construct, that error is propagated.
///
/// # Type Parameters
///
/// - `T`: The type of the values in the iterator.
/// - `E`: The error type returned by the fallible extension methods.
/// - `C`: The type of the container to be constructed.
///
/// # Examples
///
/// ```rust
#[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
/// ```
/// Handling the nested `Result<Result<C, E>, C::Error>` may be cumbersome. Consider using
/// `fluent_result::nested::FlattenErr::flatten_err` or `fluent_result::nested::BoxErr::box_err` (from the
/// `fluent_result` crate) to flatten the error type for more ergonomic handling. Alternatively, if both error types
/// are convertable to the return type of the scope using [`From`], you can simply use two instances of the `??`
/// operator.
/// 
/// ```rust
#[doc = include_doc::function_body!("tests/doc/result.rs", double_question_mark_example, [])]
/// ```
impl<T, E, C> TryFromIterator<Result<T, E>> for Result<C, C::Error>
where
    C: TryFromIterator<T>,
{
    type Error = E;

    /// Converts an iterator of `Result<A, E>` into a `Result<V, E>`.
    ///
    /// # Examples
    ///
    /// ```rust
    #[doc = include_doc::function_body!("tests/doc/result.rs", try_from_iter_result_example, [])]
    /// ```
    fn try_from_iter<I: IntoIterator<Item = Result<T, E>>>(into_iter: I) -> Result<Self, Self::Error> {
        let mut extractor: ExtractErr<_, _> = into_iter.into_iter().into();

        let try_from_result = C::try_from_iter(&mut extractor);

        match (extractor.error, try_from_result) {
            (Some(e), _) => Err(e),
            (None, Err(e)) => Ok(Err(e)),
            (None, Ok(v)) => Ok(Ok(v)),
        }
    }
}
