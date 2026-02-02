<!-- Markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.18.0] - 2026-02-02

### Added

- Added helper constructors for `CapacityError`:
  - `ensure_fits`
  - `ensure_fits_in`
  - `ensure_fits_into`
  - `collect_overflowed`
  - `collect_underflowed`
  - `extend_overflowed`
- Added helper constructors for `CollectError`:
  - `ensure_fits_in`
  - `ensure_fits_into`
  - `collect_overflowed`
  - `collect_underflowed`
  - `extend_overflowed`
- Added `ExtendError` type for `TryExtend` operations. Unlike `CollectError`, it has no `collected`
  field because `TryExtend` adds items directly to the target collection (basic error guarantee).
- Added `ExtendError::ensure_fits_into` helper for bounds checking in extend operations.
- Added `RemainingCap` and `FixedCap` implementations for `Vec`.

- Implemented `TryFrom<PartialArray<T, N>>` for `[T; N]` to allow converting a full `PartialArray` (e.g. from an overflow error) into an array.

### Changed

- **Breaking:** `TupleExtendError` is no longer generic over the `Left` and `Right` error types. `TryExtend::Error` for tuples is now `Either<TupleExtendError<...>, TupleExtendError<...>>`.
  - Migration: Remove explicit error type parameters if you were specifying them.
- **Breaking:** `UnzipError` is no longer an enum with side-specific variants. It is now a struct `UnzipError<Failed, Partial, I>`. `TryUnzip` now returns `Either<UnzipError<FromA, FromB, Self>, UnzipError<FromB, FromA, Self>>`.
  - Migration: Match on `Either` to determine which side failed, then access fields directly on the `UnzipError` struct.
- **Breaking:** Renamed `UnzipError::successful` to `UnzipError::partial` to better reflect its purpose as a partial collection from the non-failing side.
  - Migration: Rename usages of `err.successful` to `err.partial`.
- **Breaking:** Renamed `unevaluated` field to `pending` in `UnzipError` and `TupleExtendError`.
  - Migration: Rename `err.unevaluated` to `err.pending`.
  - Note: `TupleExtendError` generic parameter `U` (Unevaluated) is now `P` (Pending).
- **Breaking:** Renamed `CollectionError` to `CollectError`.
  - Migration: Update all references from `CollectionError` to `CollectError`.
- **Breaking:** Renamed `CollectError::iterator` field to `CollectError::remain`.
  - Migration: Update all references from `CollectError::iterator` to `CollectError::remain`.
- **Breaking:** Changed `ArrayVec` `TryFromIterator` and `TryExtend` implementations to return `ArrayVec` instead of `Vec` in `CollectError`.
  - Migration: Update error handling to expect `ArrayVec` in the `collected` field.
- **Breaking:** Changed `CollectError::overflowed` to `extend_overflowed`.
  - Migration: Update error handling to use `extend_overflowed` ctor.
- **Breaking:** `TryExtend` implementations now return `ExtendError` instead of `CollectError`.
  - `ExtendError` has no `collected` field (items go directly into the target collection).
  - Migration: Update error handling to use `ExtendError` instead of `CollectError`.
    The `error` and `remain` fields are still available.
- **Breaking:** Moved capacity related types (`CapacityError`, `CapacityErrorKind`, `FixedCap`, `RemainingCap`) from `errors` to `errors::capacity`.
  - Migration: Update imports to `collect_failable::errors::capacity::*`.
- **Breaking:** Renamed `RemainingSize` trait to `RemainingCap` and `MaxSize` trait to `FixedCap`.
  - Migration: Update all references from `RemainingSize` to `RemainingCap` and `MaxSize` to `FixedCap`.
- **Breaking:** Moved collision related types (`Collision`) from `errors` to `errors::collision`.
  - Migration: Update imports to `collect_failable::errors::collision::*`.
- **Breaking:** Renamed `TupleExtensionError` to `TupleExtendError`.
  - Migration: Update all references from `TupleExtensionError` to `TupleExtendError`.
- Refactored `ExtendError`, `CollectError`, `ResultCollectError`, `UnzipError`, and `TupleExtendError` to use conditional function definitions for `new` and `into_data` instead of inline `cfg` blocks.
- Removed `alloc` dependency from `unsafe`, `tuple`, and `arrayvec` features. These features now work in `no_std` environments without an allocator (error types will store data inline).
- **Breaking:** Renamed `tuple` feature to `tuples`.
  - Migration: Update `Cargo.toml` to use `tuples` instead of `tuple` in dependencies or feature lists.

### Fixed

- Added missing `alloc` feature dependency to several features (`unsafe`, `tuple`, `hashbrown`, `indexmap`, `arrayvec`) whose error types require allocation.
- Fixed incorrect `CapacityError` capacity reported when collecting into a fixed-size array overflows (via `PartialArray`). It now correctly reports the array's capacity instead of `SizeHint::ZERO`.

### Removed

- **Breaking:** Removed export of `PartialArray` type in the `errors` module. It is now only available under `errors::partial_array::PartialArray`.
  - Migration: Update all references from `PartialArray` to `errors::partial_array::PartialArray`.
- **Breaking:** Moved `SizeHint` export from crate root to `errors` module.
  - Migration: Update all references from `collect_failable::SizeHint` to `collect_failable::errors::SizeHint`.
- **Breaking:** Moved `Either` export from crate root to `errors` module.
  - Migration: Update all references from `collect_failable::either::Either` to `collect_failable::errors::either::Either`.
- **Breaking:** `TryExtendSafe` no longer extends `TryExtend`. It now has its own `Error` associated type.
  - This reflects the semantic difference: `TryExtend::Error` may have an empty `collected` field
    (items were added to target), while `TryExtendSafe::Error` includes rolled-back items.
  - Migration: Update any manual `TryExtendSafe` implementations to add `type Error = ...`.
- **Breaking:** Removed `TupleExtendError` new. This type is not intended for user construction.
  - Migration: Do not internally construct `TupleExtendError`.

## [0.17.1] - 2026-01-25

### Changed

- Updated crate edition to 2024.

## [0.17.0] - 2026-01-25

### Changed

- **Breaking:** Removed `DoubleEndedIterator` implementation for `PartialArray`.
- **Breaking:** Updated minimum supported Rust version (MSRV) to 1.93

## [0.16.0] - 2026-01-25

### Added

- Added `MaxSize` trait to expose static maximum capacity from collection types.
- Added `RemainingSize` trait (with `remaining_size` method) to expose dynamic remaining capacity from collection types.
- Added `RemainingSize` and `MaxSize` implementations for `ArrayVec` and `Array`.
- Added `Debug` implementations for `CollectionErrorData`, `ResultCollectionErrorData`, and `UnzipErrorData`.
- `TryFromIterator` implementations for hash-based collections (`HashMap`, `HashSet`, `IndexMap`, `IndexSet`, and `hashbrown` variants) now support custom hashers via the `S: BuildHasher + Default` bound.

### Changed

- **Breaking:** Renamed `Capacity` trait to `RemainingSize` and its `capacity_hint` method to `remaining_size`.
  - Migration: Update all references from `Capacity` to `RemainingSize` and from `capacity_hint()` to `remaining_size()`.
- **Breaking:** Changed `ArrayVec::try_extend_one` to return `crate::errors::CapacityError` instead of `arrayvec::CapacityError`.
  - Migration: Update error handling to expect `crate::errors::CapacityError`.
- **Breaking:** Changed all hash and set implementations that can take a `S` (hasher) to require `S: BuildHasher + Default` instead of using the default hasher (`RandomState`). This breaks some type-inference.
  - Migration: Specify the hasher type explicitly, e.g. `HashMap<K, V, RandomState>` or use `TryCollectEx`.
- **Breaking:** Changed `TryFromIterator` for `[T; N]` to return `CollectionError<I::IntoIter, PartialArray<T, N>, CapacityError<T>>` instead of `Vec<T>`.
  - Migration: Update error handling to expect `PartialArray` in the `collected` field. You can use `PartialArray::into_iter().collect::<Vec<_>>()` to get a `Vec`.

### Removed

- **Breaking:** Removed `Capacity` trait implementations for `Vec`.

## [0.15.0] - 2026-01-14

### Added

- Added `ErrorItemProvider` trait to allow error types to provide access to their collected items without consuming the error.
- Added `Capacity` trait (with `capacity_hint` method) to expose exact size hints from collection types.
- Benchmarks for `try_extend` and `try_extend_safe`.

### Changed

- **Breaking:** Renamed `CapacityMismatch` to `CapacityError`.
  - Migration: Update all references from `CapacityMismatch` to `CapacityError`.
- **Breaking:** Renamed `MismatchKind` to `CapacityErrorKind`.
  - Migration: Update all references from `MismatchKind` to `CapacityErrorKind`.
- **Breaking:** Renamed `ItemCollision` to `Collision`.
  - Migration: Update all references from `ItemCollision` to `Collision`.
- Made `CapacityError` (previously `CapacityMismatch`) fields mutable.

### Removed

- **Breaking:** Removed `CollectionCollision` error type. Use `CollectionError` directly instead.
  - Migration: Replace `CollectionCollision` with `CollectionError<T, I, C, Collision<T>>` where appropriate.

## [0.14.0] - 2026-01-07

### Added

- Added `CollectionError::underflow` convenience constructor.
- Re-exported `size_hinter::SizeHint` type in the `errors` module.
- Added `Capacity` type with `TryFrom<RangeInclusive<usize>>` implementation to represent validated capacity ranges.
- Added `InvalidCapacity` error type returned when attempting to create an invalid `Capacity`.
- **Added `no_std` support** with `alloc` and `std` features (both enabled by default). The crate now works in `no_std` environments when the `std` feature is disabled:
  - `alloc` feature enables allocation-dependent types (`BTreeMap`, `BTreeSet`, `Result`, `Rc`, `Vec`, `Box`)
  - `std` feature enables standard library types (`HashMap`, `HashSet`)
  - All imports updated to use `core` and `alloc` equivalents where possible
  - `HashMap` and `HashSet` require `std` due to their dependency on the standard library's default hasher

### Changed

- **Breaking:** Changed `MismatchKind::Bounds` variant to use `SizeHint` instead of a raw `(usize, Option<usize>)` tuple. This provides better type safety and semantic meaning for size hint information.
  - Migration: If you pattern match on `MismatchKind::Bounds`, update from `MismatchKind::Bounds(min, max)` to `MismatchKind::Bounds(size_hint)` and use `size_hint.lower()` and `size_hint.upper()` to access the bounds.
- **Breaking:** Changed `CapacityMismatch` to use a `capacity: Capacity` field instead of separate `min`/`max` fields. This encapsulates validation logic and prevents potential overflow issues when converting `RangeInclusive` to `Range` for capacities near `usize::MAX`.
  - Migration: Replace `error.min` with `error.capacity.min` and `error.max` with `error.capacity.max`.
- **Breaking:** Changed `ResultCollectionError` iterator type from `Either<I::IntoIter, iter::Empty<...>>` to `I::IntoIter`. The `Either` wrapper was an internal implementation detail and is no longer exposed in the public API.
  - Migration: If you have code that explicitly references the error type with the full type signature, remove the `Either` wrapper. The iterator field can now be used directly without unwrapping.

## [0.13.0] - 2025-12-22

### Added

- Added `tuples` feature flag (enabled by default) that gates tuple and unzip functionality. `TupleExtensionError` and `UnzipError` require this feature.
- Re-exported `either::Either` type in the public API as `collect_failable::Either` when the `tuples` feature is enabled. Users no longer need to directly depend on the `either` crate to use error types that include `Either`.
- Added `CollectionError::bounds` and `CollectionError::overflow` convenience constructors.

### Removed

- **Breaking:** Removed methods that drop data:
  - Removed `CollectionCollision::into_item()` - use `into_data().item` or direct field access via `Deref` instead.
  - Removed `CollectionError::into_error()` - use `into_data().error` or direct field access via `Deref` instead.
- **Breaking:** Removed `len()` and `is_empty()` methods from `CollectionError` and `CollectionCollision`.
- **Breaking:** Removed `IntoIterator` implementation for `ResultCollectionError`. This was removed because it was potentially misleading, as it only iterated over the collected items and ignored the remaining iterator.
- Removed `itertools` dependency.
- **Breaking:** Removed the `utils` module and `utils` feature. The `FoldMut` trait has been extracted into its own standalone crate: [`fold_mut`](https://github.com/MaxMahem/fold_mut).
  - Migration: Add `fold_mut` as a dependency and change `use collect_failable::utils::FoldMut;` to `use fold_mut::FoldMut;`.
- Removed `TupleCollectionError` error type. Tuple collections should use `TryUnzip` or `TryExtend` instead of `TryFromIterator`.
- Removed `TryFromIterator` implementation for tuples. Use `TryUnzip::try_unzip` or `TryExtend::try_extend` instead.

### Changed

- **Breaking:** Moved all error types into the `errors` submodule. Error types are no longer re-exported at the crate root.
  - Migration: change `use collect_failable::ErrorType;` to `use collect_failable::errors::ErrorType;` or add `use collect_failable::errors::*;` to import all error types.
- **Breaking:** Refactored `TryExtendOne` to use an associated type `Item` instead of a generic type parameter. This significantly simplifies type signatures throughout the codebase, particularly reducing `UnzipError` from 5 to 3 type parameters and `UnzipSide` from 4 to 2 type parameters.
  - Migration: change `impl TryExtendOne<T>` to `impl TryExtendOne { type Item = T; }` and update bounds from `where C: TryExtendOne<T>` to `where C: TryExtendOne<Item = T>`.
- **Breaking:** Simplified `ResultCollectionError` field names for clarity and consistency:
  - `iteration_error` → `error` (the first error encountered from the iterator)
  - `collection_result` → `result` (the partial collection result)
  - `result_iter` → `iter` (the remaining iterator)
  - Removed helper methods `into_iteration_error()`, `into_collection_result()`, and `into_result_iter()`. Use `into_data()` to access all fields, or access fields directly via the `Deref` implementation.
  - Migration: Update field accesses to use new names. Replace method calls like `err.into_iteration_error()` with `err.into_data().error` or simply `err.error`.
- Made `CapacityMismatch` fields readonly. (This was considered a non-breaking change because there should be no reason to mutate the error.)
- Improved documentation for error types (`CollectionCollision`, `CollectionError`, `UnzipErrorSide`, `TupleExtensionErrorSide`) by hiding internal data structs and documenting readonly fields directly on parent types.
- Converted `TupleExtensionError` and `UnzipError` from enum to struct type for improved API consistency. `either::Either` is used to hold the error sides, with a custom inner type.

## [0.12.3] - 2025-12-19

### Fixed

- Fixed debug format assertions in `tuple_extension_error` tests to match actual `TestError` struct format.

## [0.12.1] - 2025-12-18

### Fixed

- Removed internal `utils` dependency.
- Updated dependencies to use crates.io instead of git.

## [0.12.0] - 2025-12-17

Major rework, aiming to make iter information recoverable on all errors.

### Added

- Added `TryExtendOne` trait for extending collections with a single item, providing cleaner error types and strong error guarantees.
- Added `ItemCollision<T>` error type for single-item collection collisions (maps and sets).
- Added `ReadOnlyPartialIterErr` and `PartialIterErr` for returning errors from failable collection methods.
- Added `CollectionCollision` for returning errors when a collision occurs during a collection operation.
- Added `TupleCollectionError` and `TupleExtensionError` for returning errors when a tuple collection operation fails.
- Added `CapacityMismatch` error type to replace `ExceedsCapacity`, providing more semantic information about capacity violations.

### Changed

- **Breaking:** Moved generic iterator parameter `I` from method level to trait level in `TryFromIterator`. The trait signature changed from `TryFromIterator<T>` to `TryFromIterator<I>`. This allows the error type to see the iterator type, which is useful for error recovery. Most user code remains compatible due to type inference.

Change the error types of most implementations to allow recovering the consumed data on an error.

- **Breaking:** Changed the implementation of `TryFromIterator` for all set and map types to use `CollectionCollision` instead of `KeyCollision` and `ValueCollision`.
- **Breaking:** Changed `try_unzip` error type to `UnzipError<A, B, FromA, FromB, I>`.
- **Breaking:** Changed `try_extend` error type to `TupleExtensionError<A, B, FromA, FromB, I>`.
- **Breaking:** Changed `ArrayVec` implementations to use `CollectionError<T, I::IntoIter, Vec<T>, CapacityMismatch>` instead of `ExceedsCapacity`.
- **Breaking:** Changed `TryFromIterator` for `Result<C, C::Error>` to return `ResultCollectionError<E, C, C::Error>` instead of `E`. This preserves both the iterator error and the partial collection result, allowing full recovery of all information when an error occurs.
- **Breaking:** Changed `TryFromIterator` for arrays (`[T; N]`) to return `CollectionError<T, I::IntoIter, Vec<T>, CountMismatch>` instead of `ItemCountMismatch`. This allows recovery of collected items and the remaining iterator when array collection fails due to length mismatch.
- **Breaking:** Renamed `ItemCountMismatch` to `CountMismatch`.

### Removed

- **Breaking:** Removed `KeyCollision` and `ValueCollision` error types. Use `CollectionCollision` instead, which provides a unified error type for all collection collision scenarios.
- **Breaking:** Removed `OneOf2` error type. Use `TupleCollectionError` or `TupleExtensionError` instead.
- **Breaking:** Removed `ExceedsCapacity` error type. Use `CapacityMismatch` instead.

### Migration Guide

- Use the new error types. In most cases, the original error types can be recovered via the `NewErrorType.error` field.
- Migrate implementations to use the new interface shape.

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
