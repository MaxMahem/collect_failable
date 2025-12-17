[![Build](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collect_failable/actions/workflows/docs.yml/badge.svg)](https://maxmahem.github.io/collect_failable/collect_failable/index.html)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)
[![codecov](https://codecov.io/github/MaxMahem/collect_failable/graph/badge.svg?token=6JJF59BIO3)](https://codecov.io/github/MaxMahem/collect_failable)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)

# `collect_failable`

A set of traits for collecting values into containers that must uphold invariants during construction or extension. These traits let you propagate structured errors instead of panicking or silently discarding data. Examples include preventing duplicate keys in a `HashMap` or respecting capacity limits in types like `ArrayVec`.

## Features

This crate provides several complementary traits for failable collection:

- `TryFromIterator` – build a new container from an iterator, returning an error when invariants can't be satisfied.
- `TryCollectEx` – ergonomic `collect`-style extension for iterator consumers, forwarding a call to `TryFromIterator`.
- `TryExtend` – fallible extend operations with strong and basic error guarantees variants.
- `TryExtendOne` – extend with a single item, providing cleaner error types and strong guarantees.
- `TryUnzip` – `unzip` an iterator of pairs into two fallible containers.
- `FoldMut` – `fold`-style extension building a collection via mutation rather than move.

Additionally, several implementations are provided for common and popular containers. See the [implementations](#implementations) section for more details.

### `TryFromIterator` and `TryCollectEx`

Construct a container from an iterator, with errors for invalid input. This behaves like `FromIterator` but returns `Result<Self, E>` instead of panicking or ignoring failures.

```rust
use std::collections::HashMap;
use collect_failable::{TryFromIterator, TryCollectEx};

// can be called on any type that implements TryFromIterator
let err = HashMap::try_from_iter([(1, 2), (2, 3), (1, 4), (3, 5)]).expect_err("should be Err");
assert_eq!(err.item.0, 1); // err.item is the colliding (K, V) tuple

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

Extend a collection with a single item. This trait always provides a **strong guarantee**: on failure, the collection remains unchanged.

```rust
use std::collections::HashMap;
use collect_failable::TryExtendOne;

let mut map = HashMap::new();
map.try_extend_one((1, 2)).expect("should be Ok");
map.try_extend_one((2, 3)).expect("should be Ok");

// Error type just contains the rejected item
let err = map.try_extend_one((1, 5)).expect_err("should collide");
assert_eq!(err.item, (1, 5)); // Simple ItemCollision error
assert_eq!(map.get(&1), Some(&2)); // Original value unchanged
```

**Note**: Tuples do not implement `TryExtendOne` because they cannot provide atomic single-item guarantees. Use `try_extend(std::iter::once(item))` instead.

### `TryUnzip`

Fallible equivalent of [`Iterator::unzip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip). Given an iterator of `(A, B)` items, produce two collections that implement `Default + TryExtend`, stopping on the first failure.

Allows unzipping an iterator of pairs into two collections that implement `Default` and `TryExtend`.

This is analogous to [`Zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip), except allows for failable construction.

```rust
use std::collections::{BTreeSet, HashSet};
use collect_failable::TryUnzip;

// Unzip into two different container types
let data = vec![(1, 'a'), (2, 'b'), (3, 'c')];
let (nums, chars): (BTreeSet<i32>, HashSet<char>) = data.into_iter().try_unzip().expect("should be ok");

assert_eq!(nums, BTreeSet::from([1, 2, 3]));
assert_eq!(chars, HashSet::from(['a', 'b', 'c']));
```

### Utils

Also included a series of utility functions, including:
* `fold_mut` and `try_fold_mut` for folding an iterator into a mutable accumulator. Useful for implementing `TryFromIterator`.
* `FixedSizeHint` and `FixedSizeHintEx` for hiding the size hint of an iterator in tests.
* `Identifiable` for a value who's identity can differ from its `Eq` identity, useful for telling if an equal value has been overwritten dor testing.

## Implementations

Implementations for various containers are provided.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html), [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
* [Tuple of size 2](https://doc.rust-lang.org/std/primitive.tuple.html)
* [Array](https://doc.rust-lang.org/std/primitive.array.html) (feature `unsafe`)
* [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) (feature `arrayvec`)
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html), [hashbrown::HashSet](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html) (feature `hashbrown`)
* [indexmap::IndexMap](https://docs.rs/indexmap/latest/indexmap/), [indexmap::IndexSet](https://docs.rs/indexmap/latest/indexmap/) (feature `indexmap`)

### Tuple Implementations

Tuples of size 2 implement both `TryFromIterator` and `TryExtend` when their inner types do. Errors respect the guarantee of each component, mirroring the behavior of the `std` tuple implementations—but with fallibility.

### Array Implementation

Arrays implement `TryFromIterator` for iterators that yield exactly the right number of elements. This uses `unsafe` internally and is gated behind the `unsafe` feature (enabled by default).

### Result Implementation

`TryFromIterator` is implemented for `Result<C, E>`, where `C` implements `TryFromIterator<T>`, similar to the [`FromIterator`](https://doc.rust-lang.org/std/result/enum.Result.html#impl-FromIterator%3CResult%3CA,+E%3E%3E-for-Result%3CV,+E%3E) implementation for `Result`. This allows short-circuiting collection of failable values into a container whose construction is also failable.
