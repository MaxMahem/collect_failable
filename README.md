# `collect_failable`

[![Build](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collect_failable/actions/workflows/docs.yml/badge.svg)](https://maxmahem.github.io/collect_failable/collect_failable/index.html)
[![Crates.io](https://img.shields.io/crates/v/collect_failable)](https://crates.io/crates/collect_failable)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)
[![codecov](https://codecov.io/github/MaxMahem/collect_failable/graph/badge.svg?token=6JJF59BIO3)](https://codecov.io/github/MaxMahem/collect_failable)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)

A set of traits for collecting values into containers that must uphold invariants during construction or extension. These traits let you propagate structured errors instead of panicking or silently discarding data. Examples include preventing duplicate keys in a `HashMap` or respecting capacity limits in types like `ArrayVec`.

## Traits

This crate provides several complementary traits for failable collection:

- `TryFromIterator` – failably build a container from an `IntoIterator`.
- `TryCollectEx` – failably collect an `IntoIterator` into a container.
- `TryExtend` and `TryExtendSafe` – failably extend a container with an `IntoIterator`, with different error guarantees.
- `TryExtendOne` – failable extend a container with a single item.
- `TryUnzip` – failably unzip an `IntoIterator` of pairs into two containers (requires feature `tuple`, enabled by default).
- `Capacity` - expose capacity size hints for collection types (e.g., `ArrayVec`).

Additionally, several implementations are provided for common and popular containers. See the [implementations](#implementations) section for more details.

## Installation

It's on [crates.io](https://crates.io/crates/collect_failable).

## Features

This crate provides the following optional features:

- `alloc` (default) – Enables support for allocation-dependent types (`BTreeMap`, `BTreeSet`, `Result`, `Rc`, `Vec`, `Box`). Required for most functionality. When disabled, only the core traits are available.
- `std` (default) – Enables standard library support, including `HashMap` and `HashSet` implementations. When disabled, the crate operates in `no_std` mode with `alloc`.
- `unsafe` (default) – Enables `TryFromIterator` implementations for arrays using unsafe code.
- `tuple` (default) – Enables tuple extension (`TryExtend` for tuples) and `TryUnzip` trait, requiring a public dependency on the `either` crate (re-exported as `collect_failable::Either`).
- `arrayvec` – Enables `TryFromIterator` and `TryExtend` implementations for `ArrayVec`.
- `hashbrown` – Enables `TryFromIterator` and `TryExtend` implementations for `hashbrown::HashMap` and `hashbrown::HashSet`.
- `indexmap` – Enables `TryFromIterator` and `TryExtend` implementations for `indexmap::IndexMap` and `indexmap::IndexSet`.

### No-std Support

This crate supports `no_std` environments when the `std` feature is disabled. The `alloc` feature provides allocation-dependent functionality (`BTreeMap`, `BTreeSet`, etc.) without requiring the full standard library.

**Note**: `HashMap` and `HashSet` require the `std` feature because they depend on the standard library's default hasher. For `no_std` environments, consider `BTreeMap`/`BTreeSet` (with `alloc`) or `hashbrown`/`indexmap` (with their respective features).

## Usage

### `TryFromIterator` and `TryCollectEx`

Construct a container from an iterator, with errors for invalid input. This behaves like `FromIterator` but returns `Result<Self, E>` instead of panicking or ignoring failures.

```rust
use std::collections::HashMap;
use collect_failable::{TryFromIterator, TryCollectEx};

// can be called on any type that implements TryFromIterator
let err = HashMap::try_from_iter([(1, 2), (2, 3), (1, 4), (3, 5)]).expect_err("should be Err");
assert_eq!(err.error.item.0, 1); // err.error.item is the colliding (K, V) tuple

// For `HashMap` the error contains all the data necessary to reconstruct the consumed iterator
let all_items: Vec<_> = err.into_iter().collect();
assert_eq!(all_items.len(), 4); // all 4 original items are present, though order is not guaranteed

// or collected via the TryCollectEx trait a turbofish may be necessary to disambiguate
let map = [(1, 2), (2, 3)].into_iter().try_collect_ex::<HashMap<_, _>>().expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));

// or type ascription. Note the Result type can be inferred, just not the collection type.
let map: HashMap<_, _> = [(1, 2), (2, 3)].into_iter().try_collect_ex().expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
```

### `TryExtend` and `TryExtendSafe`

Extend an existing container with items that may violate its invariants. Two different trait exposes two styles of error behavior:

- `TryExtendSafe` – **strong guarantee** on an error, the container must remain unchanged.
- `TryExtend` – **basic guarantee** the container may have partially ingested items, but must remain valid.

Use `TryExtendSafe` if you must avoid mutation on failure; otherwise, prefer the faster `TryExtend`.

```rust
use std::collections::HashMap;
use collect_failable::TryExtendSafe;

let mut map = HashMap::new();
map.try_extend_safe([(1, 2), (2, 3)]).expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));

// on a failure, the container is not modified
map.try_extend_safe([(1, 3)]).expect_err("should be Err");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
```

### `TryExtendOne`

Extend a collection with a single item. This trait always provides a **strong guarantee**: on failure, the collection remains unchanged. Implemented as a seperate trait with no default implementation due to limitations imposed by the trait definition.

### `TryUnzip`

Fallible equivalent of [`Iterator::unzip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip). Given an iterator of `(A, B)` items, produce two collections that implement `Default + TryExtend`, stopping on the first failure.

```rust
use std::collections::{BTreeSet, HashSet};
use collect_failable::TryUnzip;

// Unzip into two different container types
let data = vec![(1, 'a'), (2, 'b'), (3, 'c')];
let (nums, chars): (BTreeSet<i32>, HashSet<char>) = data.into_iter().try_unzip().expect("should be ok");

assert_eq!(nums, BTreeSet::from([1, 2, 3]));
assert_eq!(chars, HashSet::from(['a', 'b', 'c']));
```

## Implementations

Implementations for various containers are provided.

- [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html), [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html) (feature `std`, enabled by default)
- [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) (feature `alloc`, enabled by default)
- [Array](https://doc.rust-lang.org/std/primitive.array.html) (feature `unsafe`, enabled by default)
- [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) (feature `arrayvec`)
- [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html), [hashbrown::HashSet](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html) (feature `hashbrown`)
- [indexmap::IndexMap](https://docs.rs/indexmap/latest/indexmap/), [indexmap::IndexSet](https://docs.rs/indexmap/latest/indexmap/) (feature `indexmap`)

### Tuple Implementations

Tuples of arity 2 implement `TryExtend` when their inner types do (requires feature `tuple`, enabled by default). For constructing tuple collections from `IntoIterator` `TryUnzip` is available.

### Array Implementation

Arrays implement `TryFromIterator` for `IntoIterator` that yield exactly the right number of elements. This uses `unsafe` internally and is gated behind the `unsafe` feature (enabled by default).

### Result Implementation

`TryFromIterator` is implemented for `Result<C, E>`, where `C` implements `TryFromIterator<T>`, similar to the [`FromIterator`](https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E) implementation for `Result`. This allows short-circuiting collection of failable values into a container whose construction is also failable.
