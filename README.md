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
- `TryUnzip` – `unzip` an iterator of pairs into two fallible containers.
- `FoldMut` – `fold`-style extension building a collection via mutation rather than move.

Additionally, several implemenations are provided for common and popular containers. See the [implementations](#implementations) section for more details.

### `TryFromIterator` and `TryCollectEx`

Construct a container from an iterator, with errors for invalid input. This behaves like `FromIterator` but returns `Result<Self, E>` instead of panicking or ignoring failures.

```rust
use std::collections::HashMap;
use collect_failable::{TryFromIterator, TryCollectEx};

// can be called on any type that implements TryFromIterator
let err = HashMap::try_from_iter([(1, 2), (1, 3)]).expect_err("should be Err");
assert_eq!(err.key, 1);

// or collected via the TryCollectEx trait a turbofish may be necessary to disambiguate
let map = [(1, 2), (2, 3)].into_iter().try_collect_ex::<HashMap<_, _>>().expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));

// or type ascription. Note the Result type can be inferred, just not the collection type.
let map: HashMap<_, _> = [(1, 2), (2, 3)].into_iter().try_collect_ex().expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
```

### `TryExtend`

Extend an existing container with items that may violate its invariants. This trait exposes two styles of error behavior:

- `try_extend_safe` – strong guarantee: on error the container must remain unchanged.
- `try_extend` – basic guarantee: the container may have partially ingested items, but must remain valid.

Use `try_extend_safe` if you must avoid mutation on failure; otherwise prefer the faster `try_extend`.

```rust
use std::collections::HashMap;
use collect_failable::TryExtend;

let mut map = HashMap::new();
map.try_extend([(1, 2), (2, 3)]).expect("should be Ok");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));

// on a failure, the container is not modified
map.try_extend([(1, 3)]).expect_err("should be Err");
assert_eq!(map, HashMap::from([(1, 2), (2, 3)]));
```

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

### `FoldMut`

An iterator extension trait for building a collection via mutation rather than move. 

 - `fold_mut` – Fold an iterator into a mutable accumulator. 
 - `try_fold_mut` – Failable version of `fold_mut`. 

```rust
use collect_failable::FoldMut;
use std::collections::HashMap;

let pairs = vec![("a", 1), ("b", 2), ("a", 3)];

let result = pairs.into_iter().fold_mut(HashMap::new(), |map, (key, val)| {
    map.entry(key).or_insert(0);
    *map.get_mut(key).unwrap() += val;
});

assert_eq!(result.get("a"), Some(&4));
assert_eq!(result.get("b"), Some(&2));
```

### Tuple Implementations

Tuples of size 2 implement both `TryFromIterator` and `TryExtend` when their inner types do. Errors respect the guarantee of each component, mirroring the behavior of the `std` tuple implementations—but with fallibility.

### Array Implementation

Arrays implement `TryFromIterator` for iterators that yield exactly the right number of elements. This uses `unsafe` internally and is gated behind the `unsafe` feature (enabled by default).

## Implementations

Implementations for various containers are provided.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html), [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
* [Tuple of size 2](https://doc.rust-lang.org/std/primitive.tuple.html)
* [Array](https://doc.rust-lang.org/std/primitive.array.html) (feature `unsafe`)
* [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) (feature `arrayvec`)
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html), [hashbrown::HashSet](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html) (feature `hashbrown`)
* [indexmap::IndexMap](https://docs.rs/indexmap/latest/indexmap/), [indexmap::IndexSet](https://docs.rs/indexmap/latest/indexmap/) (feature `indexmap`)
