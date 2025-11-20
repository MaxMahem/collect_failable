[![Build](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/collect_failable/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/collect_failable/actions/workflows/docs.yml/badge.svg)](https://maxmahem.github.io/collect_failable/collect_failable/index.html)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)
[![codecov](https://codecov.io/github/MaxMahem/collect_failable/graph/badge.svg?token=6JJF59BIO3)](https://codecov.io/github/MaxMahem/collect_failable)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)

A trait for collecting values into a container that has an invariant to uphold and whose construction may fail.

## Features

### TryFromIterator and TryCollectEx

Allows collection of an iterator into a container that may fail to be constructed. For example, a `HashMap` may not allow duplicate keys. Or an `ArrayVec` might only be able to hold a certain number of elements.

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

### TryExtend

Allows an existing collection to be extended with values from an iterator that may fail to be constructed. Two methods are provided: `try_extend` and `try_extend_safe`. `try_extend_safe` is required, and should provide a strong error gurantee. If the method returns an error, the collection should not be modified. `try_extend`, if implemented, should provide a weak error guarantee. If the method returns an error, the collection may be modified, but should be in a valid state.

For example, for `HashMap`, `try_extend_safe` would not mutate the collection if provided with duplicate keys. `try_extend` would mutate the collection, however it should not overwrite the existing value.

`try_extend_safe` generally requires some extra allocations and checks in order to provide its gurantees, so `try_extend` or `try_collect` should be favored if these gurantees are not necessary.

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

### TryUnzip

Allows unzipping an iterator of pairs into two collections that implement `Default` and `TryExtend`.

This is analogous to [`Zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip), except allows for failable construction.

```rust
use std::collections::HashSet;
use collect_failable::TryUnzip;

let data = vec![(1, 2), (2, 3)];
let (a, b): (HashSet<i32>, HashSet<i32>) = data.into_iter().try_unzip().expect("Should be ok");

assert_eq!(a, HashSet::from([1, 2]));
assert_eq!(b, HashSet::from([2, 3]));
```

### Tuple implementations

Tuple implementations are provided for tuples of size 2 that implement `TryFromIterator` or `TryCollectEx`. They will respect the error guarantee of the inner types, if any.

### Array implementations

The array implementation allows iterators that exactly fill an array to be collected into it. It requires `unsafe` code, and is gated behind the `unsafe` feature (enabled by default).

## Implementations

Implementations for various containers are provided.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
* [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
* [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
* [Tuple of size 2](https://doc.rust-lang.org/std/primitive.tuple.html)
* [Array](https://doc.rust-lang.org/std/primitive.array.html) - requires feature `unsafe` (on by default) `TryFromIterator` only
* [ArrayVec](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html) - requires feature `arrayvec`
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) - requires feature `hashbrown`
* [hashbrown::HashSet](https://docs.rs/hashbrown/latest/hashbrown/struct.HashSet.html) - requires feature `hashbrown`
* [indexmap::IndexMap](https://docs.rs/indexmap/latest/indexmap/) - requires feature `indexmap`
* [indexmap::IndexSet](https://docs.rs/indexmap/latest/indexmap/) - requires feature `indexmap`
