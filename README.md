# collect_failable

[![CI](https://github.com/MaxMahem/collect_failable/workflows/CI/badge.svg)](https://github.com/MaxMahem/collect_failable/actions)

A trait for collecting values into a container which has an invariant to uphold and whose construction may fail.

## Features

Implementations for various containers are provided, gated behind similarly named feature flags.
* [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) - `hash_map`
* [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) - `btree_map`
* [hashbrown::HashMap](https://docs.rs/hashbrown/latest/hashbrown/struct.HashMap.html) - `hash_brown`

## Usage

```rust
use std::collections::HashMap;
use collect_failable::{TryFromIterator, FailableCollectExt};

// can be called on any type that implements TryFromIterator
let err = HashMap::try_from_iter([(1, 2), (1, 3)].into_iter());
assert!(err.is_err());
assert_eq!(err.unwrap_err().key, 1);

// or any iterator via the FailableCollectExt trait
// like normal collect a turbofish or type ascription is often necessary to disambiguate
let ok = [(1, 2), (2, 3)].into_iter().try_collect_ex::<HashMap<_, _>>();
assert!(ok.is_ok());
assert_eq!(ok.unwrap(), HashMap::from([(1, 2), (2, 3)]));
```

## License

Licensed under either of

 * [Apache License, Version 2.0](LICENSE-APACHE.md)
 * [MIT license](LICENSE-MIT.md)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
