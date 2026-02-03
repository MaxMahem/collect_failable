<!-- markdownlint-disable no-inline-html table_allowed_elements br -->

# `collect_failable`

[![Build](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collect_failable/actions/workflows/docs.yml/badge.svg)](https://maxmahem.github.io/collect_failable/collect_failable/index.html)
[![Crates.io](https://img.shields.io/crates/v/collect_failable)](https://crates.io/crates/collect_failable)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)
[![codecov](https://codecov.io/github/MaxMahem/collect_failable/graph/badge.svg?token=6JJF59BIO3)](https://codecov.io/github/MaxMahem/collect_failable)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)

A set of traits for collecting values into containers that must uphold invariants during construction or extension. These traits let you propagate structured errors instead of panicking or silently discarding data. Examples include preventing duplicate keys in a `HashMap` or respecting capacity limits in types like `ArrayVec`.

## Installation

It's on [crates.io](https://crates.io/crates/collect_failable).

## Traits

This crate provides several complementary traits for failable collection:

- [`TryFromIterator`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryFromIterator.html) and [`TryCollectEx`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryCollectEx.html) – failably build a container from an `IntoIterator`.
- [`TryExtend`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtend.html), [`TryExtendSafe`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtendSafe.html) and [`TryExtendOne`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtendOne.html) – failably extend a container with an `IntoIterator`, with different error guarantees.
- [`TryUnzip`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryUnzip.html) – failably unzip an `IntoIterator` of pairs into two containers (requires feature `tuples`, enabled by default).

Additionally, several implementations are provided for common and popular containers. See the [implementations](#implementations) section for more details.

## Error Recovery

Implementations from this crate make a special emphasis on being able to recover all data in the case of collection or extension failure. The [`CollectError`](https://docs.rs/collect_failable/latest/collect_failable/errors/struct.CollectError.html) and [`ExtendError`](https://docs.rs/collect_failable/latest/collect_failable/errors/struct.ExtendError.html) types will contain both the (potentially partially iterated) iterator, the items collected up to the point of failure (if any), and the item that caused the failure (if any), and can be converted back into an iterator if desired.

## Features

This crate provides the following optional features:

| Feature | Description | Dependencies |
| :--- | :--- | :--- |
| **Default** | Default features of the crate | `alloc`, `std`, `unsafe`, `tuples` |
| `alloc` | Enables support for allocation-dependent types (`BTreeMap`, `BTreeSet`). | - |
| `std` | Enables standard library support, including `HashMap` and `HashSet` implementations. When disabled, the crate is `no_std` compatible. | `alloc` |
| `unsafe` | Enables `TryFromIterator` implementations for arrays using unsafe code. | - |
| `tuples` | Enables tuple extension ([`TryExtend`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtend.html) for tuples) and [`TryUnzip`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryUnzip.html) trait. | [`either`](https://crates.io/crates/either) |
| `arrayvec` | Enables `TryFromIterator` and `TryExtend` implementations for [`ArrayVec`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html). | [`arrayvec`](https://crates.io/crates/arrayvec) |
| `hashbrown` | Enables `TryFromIterator` and `TryExtend` implementations for [`hashbrown::HashMap`](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) and [`hashbrown::HashSet`](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html). | `alloc`, [`hashbrown`](https://crates.io/crates/hashbrown) |
| `indexmap` | Enables `TryFromIterator` and `TryExtend` implementations for [`IndexMap`](https://docs.rs/indexmap/latest/indexmap/) and [`IndexSet`](https://docs.rs/indexmap/latest/indexmap/). | `alloc`, [`indexmap`](https://crates.io/crates/indexmap) |

### `no_std` Support

This crate supports `no_std` environments when the `std` feature is disabled. The `alloc` feature provides allocation-dependent functionality (`BTreeMap`, `BTreeSet`, etc.) without requiring the full standard library. The traits and error types are available without any features enabled.

## Usage

### [`TryFromIterator`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryFromIterator.html) and [`TryCollectEx`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryCollectEx.html)

Construct a container from an iterator, with errors for invalid input. This behaves like [`FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) but returns `Result<Self, E>` instead of panicking or ignoring failures.

```rust
use std::collections::BTreeMap;
use collect_failable::TryFromIterator;

// try_from_iter is the core method - works on any TryFromIterator implementor
let map = BTreeMap::try_from_iter([(1, "a"), (2, "b")]).expect("no duplicates");
assert_eq!(map, BTreeMap::from([(1, "a"), (2, "b")]), "should contain all values");

// duplicate keys produce an error containing the colliding item
let err = BTreeMap::try_from_iter([(1, "a"), (2, "b"), (1, "c")]).expect_err("duplicate key");
assert_eq!(err.error.item, (1, "c"), "should contain the colliding item");

// errors contain all data needed to reconstruct the consumed iterator
// order is: rejected item, then collected items, then remaining iterator
let recovered: Vec<_> = err.into_iter().collect();
assert_eq!(recovered, [(1, "c"), (1, "a"), (2, "b")]);
```

[`TryCollectEx`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryCollectEx.html) provides a more convenient alternative, similar to [`collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect):

```rust
use std::collections::HashMap;
use collect_failable::TryCollectEx;

let map: HashMap<_, _> = [(1, "a"), (2, "b")].into_iter().try_collect_ex().unwrap();
assert_eq!(map, HashMap::from([(1, "a"), (2, "b")]));
```

### [`TryExtend`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtend.html) and [`TryExtendSafe`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtendSafe.html)

Extend an existing container with items that may violate its invariants. Two different trait exposes two styles of error behavior:

- [`TryExtendSafe`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtendSafe.html) – **strong guarantee** On an error, the container must remain unchanged.
- [`TryExtend`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtend.html) – **basic guarantee** The container may have partially ingested items, but must remain valid.

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

### [`TryExtendOne`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryExtendOne.html)

Extend a collection with a single item. This trait always provides a **strong guarantee**. On failure, the collection remains unchanged. Implemented as a seperate trait with no default implementation due to limitations imposed by the trait definition.

### [`TryUnzip`](https://docs.rs/collect_failable/latest/collect_failable/trait.TryUnzip.html)

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

| Feature | Description |
| :--- | :--- |
| `std` | `TryFromIterator` and `TryExtend` family of traits for [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html). |
| `alloc` | `TryFromIterator` and `TryExtend` family of traits for [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) and [`BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html).<br>`TryFromIterator` for iterators of `Result<T, E>` |
| `arrayvec` | `TryFromIterator` and `TryExtend` family of traits for [`ArrayVec`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html). |
| `hashbrown` | `TryFromIterator` and `TryExtend` family of traits for [`HashMap`](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) and [`HashSet`](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html). |
| `indexmap` | `TryFromIterator` and `TryExtend` family of traits for [`IndexMap`](https://docs.rs/indexmap/latest/indexmap/struct.IndexMap.html) and [`IndexSet`](https://docs.rs/indexmap/latest/indexmap/struct.IndexSet.html). |
| `tuples` | `TryExtend` and `TryUnzip` for Tuples of arity 2. |
| `unsafe` | `TryFromIterator` for Arrays. |
