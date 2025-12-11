# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

 - Added `ReadOnlyPartialIterErr` and `PartialIterErr` for returning errors from failable collection methods.
 - Added `CollectionCollision` for returning errors when a collision occurs during a collection operation.
 - Added `TupleCollectionError` and `TupleExtensionError` for returning errors when a tuple collection operation fails.

### Changed

 - **Breaking:** Moved generic iterator parameter `I` from method level to trait level in `TryFromIterator`. The trait signature changed from `TryFromIterator<T>` to `TryFromIterator<T, I>`. This allows the error type to see the iterator type, which is useful for error recovery. Most user code remains compatible due to type inference.

Change the error types of most implementations to allow recovering the consumed data on an error.

 - **Breaking:** Changed the implementation of `TryFromIterator` for all set and map types to use `CollectionCollision` instead of `KeyCollision` and `ValueCollision`.
 - **Breaking:** Changed `try_unzip` error type to `UnzipError<A, B, FromA, FromB, I>`.
 - **Breaking:** Changed `try_extend` error type to `TupleExtensionError<A, B, FromA, FromB, I>`.
 - **Breaking:** Changed `ArrayVec` implementations to use `CollectionError<T, I::IntoIter, Vec<T>, ExceedsCapacity>` instead of `ExceedsCapacity`.

### Removed

 - **Breaking:** Removed `KeyCollision` and `ValueCollision` error types. Use `CollectionCollision` instead, which provides a unified error type for all collection collision scenarios.
 - **Breaking:** Removed `OneOf2` error type. Use `TupleCollectionError` or `TupleExtensionError` instead.

### Migration Guide

Use the new error types. In most cases, the original error types can be recovered via the `NewErrorType.error` field.

## [0.11.1] - 2025-12-02

### Fixed

 - Fixed a problem with `utils` module internal visibility.

## [0.11.0] - 2025-12-02

### Changed

 - Moved the `FoldMut` trait into the `utils` module.
 - Split `TryExtend` trait into `TryExtend` (basic guarantee) and `TryExtendSafe` (strong guarantee).
 - `TryExtendSafe` is now a supertrait of `TryExtend`.
 - Tuples now only implement `TryExtend` as they cannot provide strong error guarantees.

### Added

 - Exposed the `FixedSizeHint` and `FixedSizeHintEx` traits. Useful for testing.
 - Exposed the `Identifiable` trait. Useful for testing.

## [0.10.0] - 2025-11-28

### Added

 - Added `TryFromIterator` implementations for `Result<C, E>`.

### Changed

 - Improvement to `ArrayVec::try_extend_safe` algorithm.

## [0.9.0] - 2025-11-22

### Added

 - Added `FoldMut` trait, accumulating values via mutation rather than move.
 - Added `IsVariant`, `TryUnwrap`, and `Unwrap` derives for `OneOf2`.

### Changed

 - Changed `TryCollectEx` trait to be a super trait of `Iterator`.

## [0.8.0] - 2025-11-20

### Added

 - Added `TryFromIterator` and `TryExtend` implementations for `ArrayVec`.
 - Added `TryUnzip` trait for `Iterator`s of tuples, allowing for failable construction of multiple collections at once, similar to `Unzip`.
 - Added `TryFromIterator` and `TryExtend` implementations for 2 value tuples.
 - Added `TryFromIterator` implementations for arrays.
 - Added `unsafe` feature gate (enabled by default).
 - Added `ExceedsCapacity` error type.

## [0.7.2] - 2025-11-15

### Fixed/Changed

 - Ensured the display trait was implemented for `ValueCollision<T>`
 - Removed display of the `key`/`value` fields from `KeyCollision<T>` and `ValueCollision<T>` to avoid requiring `Debug` or `Display` on the `K` and `V` types.

## [0.7.1] - 2025-11-15

### Fixed

 - Fixed optional dependencies

## [0.7.0] - 2025-11-15

### Added

 - Added implementations for `TryFromIterator` and `TryExtend` for `HashSet`, `BTreeSet`, `hashbrown::HashSet`, and `indexmap::IndexSet`.

### Removed

 - Removed `BtreeMap` and `HashMap` features. Those implementations are now present by default.

## [0.6.0] - 2025-11-15

### Added

 - Added `Ord`, `PartialOrd`, `Eq`, and `PartialEq` derives for `KeyCollision<K>`.

### Changed

 - Changed the `TryExtend` trait implementations to return a `KeyCollection<K>`.
 - Added `BuildHasher` generic parameter to `TryExtend` implementations.

### Removed

 - Removed the `NonUniqueKey` error type.

## [0.5.0] - 2025-11-15

### Added

 - Added the `TryExtend` trait, including implementations for `HashMap`, `BTreeMap`, `hashbrown::HashMap`, and `indexmap::IndexMap`.

### Changed

 - Removed `Hash` and `Eq` requirement from `BTreeMap` `TryFromIterator` implementation.

## [0.4.0] - 2025-11-12

### Changed

 - Renamed the `hash_brown` feature to `hashbrown`

## [0.3.1] - 2025-11-12

### Added

 - Added the `indexmap` feature

## [0.3.0] - 2025-10-28

### Changed
 
 - Renamed the FailableCollectEx trait to TryCollectEx

## [0.2.0] - 2025-10-28

### Changed

 - Changed the try_from_iterator to consume a IntoIterator instead of a Iterator

## [0.1.0] - 2025-10-27

Initial release
