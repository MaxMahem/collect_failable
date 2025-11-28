# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
