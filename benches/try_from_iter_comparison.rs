use arrayvec::ArrayVec;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use display_as_debug::wrap::TypeName;
use indexmap::{IndexMap, IndexSet};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hint::black_box;

use collect_failable::TryFromIterator;

/// Generate (key, value) pairs for map benchmarks
fn gen_pairs(size: usize) -> Vec<(usize, usize)> {
    (0..size).map(|i| (i, i * 2)).collect()
}

/// Generate scalar values for set benchmarks  
fn gen_scalars(size: usize) -> Vec<usize> {
    (0..size).collect()
}

/// Generic benchmark comparing from_iter and try_from_iter
///
/// # Type Parameters
/// - `C`: The collection type to benchmark
/// - `V`: The item type that the collection extends with
/// - `F`: A function that generates `Vec<V>` of a given size
///
/// The benchmark group name is automatically derived from the collection type `C`.
fn bench_from_iter_comparison<C, V, F>(c: &mut Criterion, sizes: &[usize], generate_data: F)
where
    C: FromIterator<V> + TryFromIterator<Vec<V>>,
    C::Error: std::fmt::Debug,
    F: Fn(usize) -> Vec<V>,
{
    let mut group = c.benchmark_group(format!("{:?}", TypeName::<C>::SHORT));

    for &size in sizes {
        group.throughput(Throughput::Elements(size as u64));

        let name = "from_iter";
        group.bench_with_input(BenchmarkId::new(name, size), &size, |b, &size| {
            b.iter_batched(
                || generate_data(size),
                |data| {
                    let collection = C::from_iter(black_box(data));
                    black_box(collection)
                },
                BatchSize::SmallInput,
            );
        });

        let name = "try_from_iter";
        group.bench_with_input(BenchmarkId::new(name, size), &size, |b, &size| {
            b.iter_batched(
                || generate_data(size),
                |data| {
                    let result = C::try_from_iter(black_box(data));
                    black_box(result)
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

const BENCH_SIZES: [usize; 4] = [100, 1000, 5000, 10000];

macro_rules! define_bench {
    ($fn_name:ident, $collection:ty, $sizes:expr, $gen_fn:expr) => {
        fn $fn_name(c: &mut Criterion) {
            bench_from_iter_comparison::<$collection, _, _>(c, $sizes, $gen_fn);
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
    name = try_from_iter_map;
    config = Criterion::default();
    targets = bench_hashmap, bench_btreemap, bench_indexmap, bench_hashbrown
);

criterion_group!(
    name = try_from_iter_set;
    config = Criterion::default();
    targets = bench_hashset, bench_btreeset, bench_indexset, bench_hashbrown_set
);

criterion_group!(
    name = try_from_iter_vec;
    config = Criterion::default();
    targets = bench_arrayvec
);

criterion_main!(try_from_iter_map, try_from_iter_set, try_from_iter_vec);
