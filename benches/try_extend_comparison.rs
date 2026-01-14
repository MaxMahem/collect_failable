use arrayvec::ArrayVec;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use indexmap::{IndexMap, IndexSet};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hint::black_box;

use collect_failable::{TryExtend, TryExtendSafe};

/// Generate (key, value) pairs for map benchmarks
fn gen_pairs(size: usize) -> Vec<(usize, usize)> {
    (0..size).map(|i| (i, i * 2)).collect()
}

/// Generate scalar values for set benchmarks  
fn gen_scalars(size: usize) -> Vec<usize> {
    (0..size).collect()
}

/// Get a short name for a type
fn get_short_name<T>() -> &'static str {
    let type_name = std::any::type_name::<T>();
    type_name
        .rsplit("::")
        .next()
        .unwrap_or(type_name)
        // Remove generic parameters for cleaner names
        .split('<')
        .next()
        .unwrap_or(type_name)
}

/// Generic benchmark comparing extend, try_extend, and try_extend_safe
///
/// # Type Parameters
/// - `C`: The collection type to benchmark
/// - `V`: The item type that the collection extends with
/// - `F`: A function that generates `Vec<V>` of a given size
///
/// The benchmark group name is automatically derived from the collection type `C`.
fn bench_extend_comparison<C, V, F>(c: &mut Criterion, sizes: &[usize], generate_data: F)
where
    C: Default + Extend<V> + TryExtend<Vec<V>> + TryExtendSafe<Vec<V>>,
    C::Error: std::fmt::Debug,
    F: Fn(usize) -> Vec<V>,
{
    let group_name = get_short_name::<C>();
    let mut group = c.benchmark_group(group_name);

    for &size in sizes {
        group.throughput(Throughput::Elements(size as u64));

        macro_rules! bench_method {
            ($method:ident) => {
                let name = stringify!($method);
                group.bench_with_input(BenchmarkId::new(name, size), &size, |b, &size| {
                    b.iter_batched(
                        || (C::default(), generate_data(size)),
                        |(mut collection, data)| {
                            _ = collection.$method(black_box(data));
                            black_box(collection)
                        },
                        BatchSize::SmallInput,
                    );
                });
            };
        }

        bench_method!(extend);
        bench_method!(try_extend);
        bench_method!(try_extend_safe);
    }

    group.finish();
}

const BENCH_SIZES: [usize; 4] = [100, 1000, 5000, 10000];

macro_rules! define_bench {
    ($fn_name:ident, $collection:ty, $sizes:expr, $gen_fn:expr) => {
        fn $fn_name(c: &mut Criterion) {
            bench_extend_comparison::<$collection, _, _>(c, $sizes, $gen_fn);
        }
    };
}

// Map benchmarks
define_bench!(bench_hashmap, HashMap<usize, usize>, &BENCH_SIZES, gen_pairs);
define_bench!(bench_btreemap, BTreeMap<usize, usize>, &BENCH_SIZES, gen_pairs);
define_bench!(bench_indexmap, IndexMap<usize, usize>, &BENCH_SIZES, gen_pairs);
define_bench!(bench_hashbrown, hashbrown::HashMap<usize, usize>, &BENCH_SIZES, gen_pairs);

// Set benchmarks
define_bench!(bench_hashset, HashSet<usize>, &BENCH_SIZES, gen_scalars);
define_bench!(bench_btreeset, BTreeSet<usize>, &BENCH_SIZES, gen_scalars);
define_bench!(bench_indexset, IndexSet<usize>, &BENCH_SIZES, gen_scalars);
define_bench!(bench_hashbrown_set, hashbrown::HashSet<usize>, &BENCH_SIZES, gen_scalars);

// Vec-like benchmarks
define_bench!(bench_arrayvec, ArrayVec<usize, 1000>, &[100, 500, 1000], gen_scalars);

criterion_group!(
    name = try_extend_map;
    config = Criterion::default();
    targets = bench_hashmap, bench_btreemap, bench_indexmap, bench_hashbrown
);

criterion_group!(
    name = try_extend_set;
    config = Criterion::default();
    targets = bench_hashset, bench_btreeset, bench_indexset, bench_hashbrown_set
);

criterion_group!(
    name = try_extend_vec;
    config = Criterion::default();
    targets = bench_arrayvec
);

criterion_main!(try_extend_map, try_extend_set, try_extend_vec);
