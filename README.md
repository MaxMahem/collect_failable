# collect_failable

[![CI](https://github.com/MaxMahem/collect_failable/workflows/CI/badge.svg)](https://github.com/MaxMahem/collect_failable/actions)
![GitHub License](https://img.shields.io/github/license/maxmahem/collect_failable)
[![dependency status](https://deps.rs/repo/github/maxmahem/collect_failable/status.svg)](https://deps.rs/repo/github/maxmahem/collect_failable)

A trait for collecting values into a container that has an invariant to uphold and whose construction may fail.

## Features

Implementations for various containers are provided, gated behind similarly named feature flags.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) - `hash_map`
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) - `btree_map`
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) - `hash_brown`

## Usage

```rust
use std::collections::HashMap;
use collect_failable::{TryFromIterator, TryCollectEx};

// can be called on any type that implements TryFromIterator
let err = HashMap::try_from_iter([(1, 2), (1, 3)].into_iter());
assert!(err.is_err());
assert_eq!(err.unwrap_err().key, 1);

// or any iterator via the TryCollectEx trait
// like normal collect a turbofish or type ascription is often necessary to disambiguate
let ok = [(1, 2), (2, 3)].into_iter().try_collect_ex::<HashMap<_, _>>();
assert!(ok.is_ok());
assert_eq!(ok.unwrap(), HashMap::from([(1, 2), (2, 3)]));
```
