/// An error type that can be one of two errors.
///
/// This is used when an operation can fail with one of two possible errors, for example when
/// unzipping an iterator into two collections, where either collection might fail to extend.
#[derive(
    Debug, PartialEq, Eq, thiserror::Error, derive_more::TryUnwrap, derive_more::IsVariant, derive_more::Unwrap,
)]
pub enum OneOf2<ErrA, ErrB> {
    /// The operation failed with the first error.
    #[error(transparent)]
    A(ErrA),
    /// The operation failed with the second error.
    #[error(transparent)]
    B(ErrB),
}
