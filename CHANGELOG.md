# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
