# collect_failable

[![CI](https://github.com/MaxMahem/collect_failable/workflows/CI/badge.svg)](https://github.com/MaxMahem/collect_failable/actions)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)
[![codecov](https://codecov.io/github/MaxMahem/collect_failable/graph/badge.svg?token=6JJF59BIO3)](https://codecov.io/github/MaxMahem/collect_failable)

A trait for collecting values into a container that has an invariant to uphold and whose construction may fail.

## Features

Implementations for various containers are provided, gated behind similarly named feature flags.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) - `hash_map`
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) - `btree_map`
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) - `hashbrown`
* [indexmap::IndexMap](https://docs.rs/indexmap/latest/indexmap/) - `indexmap`

## Usage

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
