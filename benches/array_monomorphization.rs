//! Benchmark comparing type-erased vs naive generic array TryFromIterator implementations.

use core::mem::MaybeUninit;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

use collect_failable::TryFromIterator;

/// Naive implementation: fully generic over both T and N, monomorphized for every (T, N) pair.
mod naive {
    use core::mem::MaybeUninit;

    #[derive(Debug)]
    pub struct ArrayError;

    #[inline]
    pub fn try_from_iter<const N: usize, T>(iter: impl IntoIterator<Item = T>) -> Result<[T; N], ArrayError> {
        let mut array: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        let mut iter = iter.into_iter();
        let mut count = 0;

        for slot in array.iter_mut() {
            match iter.next() {
                Some(item) => {
                    slot.write(item);
                    count += 1;
                }
                None => {
                    for init in &mut array[..count] {
                        unsafe {
                            init.assume_init_drop();
                        }
                    }
                    return Err(ArrayError);
                }
            }
        }

        if iter.next().is_some() {
            for init in &mut array {
                unsafe {
                    init.assume_init_drop();
                }
            }
            return Err(ArrayError);
        }

        Ok(unsafe { core::mem::transmute_copy(&array) })
    }
}

fn gen_scalars(size: usize) -> Vec<usize> {
    (0..size).collect()
}

macro_rules! bench_array {
    ($group:expr, $size:literal) => {{
        let data = gen_scalars($size);
        $group.throughput(Throughput::Elements($size as u64));

        $group.bench_with_input(BenchmarkId::new("erased", $size), &$size, |b, _| {
            b.iter_batched(|| data.clone(), |d| black_box(<[usize; $size]>::try_from_iter(black_box(d))), BatchSize::SmallInput);
        });

        $group.bench_with_input(BenchmarkId::new("naive", $size), &$size, |b, _| {
            b.iter_batched(
                || data.clone(),
                |d| black_box(naive::try_from_iter::<$size, usize>(black_box(d))),
                BatchSize::SmallInput,
            );
        });
    }};
}

// Pathological case: Pure computation iterator (Map)
// This interacts poorly with dynamic dispatch because the work per item is tiny,
// so the dispatch overhead is relatively larger. It also tests if the optimization
// barrier prevents vectorization/unrolling that the naive impl might get.
macro_rules! bench_array_pathological {
    ($group:expr, $size:literal) => {{
        $group.throughput(Throughput::Elements($size as u64));

        $group.bench_with_input(BenchmarkId::new("erased", $size), &$size, |b, &s| {
            b.iter(|| {
                // Use a range map - very cheap to create, very cheap next()
                let iter = (0..s).map(|i| i * 2);
                black_box(<[usize; $size]>::try_from_iter(iter))
            });
        });

        $group.bench_with_input(BenchmarkId::new("naive", $size), &$size, |b, &s| {
            b.iter(|| {
                let iter = (0..s).map(|i| i * 2);
                black_box(naive::try_from_iter::<$size, usize>(iter))
            });
        });
    }};
}

fn bench_array_try_from_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_vec_iter");

    bench_array!(group, 32);
    bench_array!(group, 128);
    bench_array!(group, 512);
    bench_array!(group, 1024);

    group.finish();
}

fn bench_array_pathological_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_pathological_map_iter");

    bench_array_pathological!(group, 32);
    bench_array_pathological!(group, 128);
    bench_array_pathological!(group, 512);
    bench_array_pathological!(group, 1024);

    group.finish();
}

criterion_group!(
    name = array_dispatch_vs_monomorphization;
    config = Criterion::default();
    targets = bench_array_try_from_iter, bench_array_pathological_iter
);
criterion_main!(array_dispatch_vs_monomorphization);
