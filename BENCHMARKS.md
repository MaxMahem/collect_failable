# Benchmark Guide

This document explains how to run and interpret the benchmarks for `collect_failable`.

## Running Benchmarks

### Run all benchmarks

```bash
cargo bench
```

### Run specific benchmark group

```bash
cargo bench --bench try_extend_comparison
```

### Run benchmarks matching a pattern

```bash
# Only HashMap benchmarks
cargo bench HashMap

# Only try_extend (not try_extend_safe)
cargo bench "try_extend/"

# Only collision detection
cargo bench "Collision Detection"
```

### Generate HTML reports

```bash
cargo bench
# Open target/criterion/report/index.html in your browser
```

## Benchmark Groups

### 1. HashMap

Compares `try_extend` vs `try_extend_safe` for `HashMap` with varying sizes (10, 100, 1000, 10000 elements).

**Purpose**: Measure basic performance difference between the two methods on the most common collection type.

### 2. BTreeMap

Same comparison but for `BTreeMap`.

**Purpose**: See if performance characteristics differ for ordered vs unordered maps.

### 3. Collision Detection

Measures performance when a collision actually occurs partway through extension.

**Purpose**: Understand the overhead of error handling and rollback in `try_extend_safe`.

### 4. Extend Populated Map

Benchmarks extending an already-populated map with non-overlapping keys.

**Purpose**: Measure the impact of existing map size on extension performance.

## Expected Results

### `try_extend` (Basic Guarantee)

- **Faster** in the success case (no collisions)
- **May modify** the collection before encountering an error
- Lower overhead for collision handling

### `try_extend_safe` (Strong Guarantee)

- **Potentially slower** due to buffering or defensive validation
- **Never modifies** the collection on error
- Higher overhead but stronger guarantees

## Interpreting Results

Criterion will output:

- **Time per iteration**: How long each benchmark takes
- **Throughput**: Elements processed per second
- **Change detection**: Whether performance has regressed or improved since last run

### Look for

1. **Percentage difference** between `try_extend` and `try_extend_safe`
2. **Scaling behavior** as input size increases
3. **Collision overhead** compared to success cases

## Customizing Benchmarks

Edit [`benches/try_extend_comparison.rs`](file:///c:/Users/maxma/source/repos/Rust/collect_failable/benches/try_extend_comparison.rs) to:

- Add more collection types (e.g., `HashSet`, `IndexMap`)
- Test different data distributions
- Vary collision rates
- Add benchmarks for `TryExtendOne`
- Compare with standard library `extend` for reference

## Example Output

```txt
HashMap/try_extend/10    time:   [123.45 ns 125.67 ns 127.89 ns]
HashMap/try_extend_safe/10
                         time:   [145.67 ns 148.23 ns 150.89 ns]
                         change: [+18.2% +18.9% +19.6%] (p = 0.00 < 0.05)
                         Performance has regressed.
```

This shows `try_extend_safe` is about 19% slower for 10 elements in this example.

## CI Integration

To run benchmarks in CI without saving baselines:

```bash
cargo bench --no-fail-fast -- --test
```

This runs benchmarks in "test mode" which is faster but doesn't do statistical analysis.
