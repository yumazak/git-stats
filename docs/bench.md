# Benchmarking Guide

This document describes how to run performance benchmarks for kodo.

## Prerequisites

- Rust toolchain (1.93+)
- A git repository to benchmark against

## Quick Start

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark group
cargo bench -- commits_in_range
cargo bench -- collect_stats
```

## Repository Selection

Benchmarks use the following priority for selecting a repository:

1. **config.json** - First repository from `~/.config/kodo/config.json`
2. **Environment variable** - `KODO_BENCH_REPO=/path/to/repo`
3. **Current directory** - Falls back to `.`

### Using a specific repository

```bash
KODO_BENCH_REPO=/path/to/large/repo cargo bench
```

## Available Benchmarks

### commits_in_range

Measures `Repository::commits_in_range()` performance with different day ranges.

```bash
# All day ranges (7, 30, 90)
cargo bench -- commits_in_range

# Specific day range
cargo bench -- commits_in_range/7
cargo bench -- commits_in_range/30
cargo bench -- commits_in_range/90
```

### collect_stats

Measures `collect_stats()` function with daily and weekly aggregation.

```bash
cargo bench -- collect_stats
cargo bench -- collect_stats/daily
cargo bench -- collect_stats/weekly
```

## Benchmark Reports

HTML reports are generated in `target/criterion/`. Open `target/criterion/report/index.html` in a browser to view detailed results.

### Comparing results

```bash
# Save baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main
```

## Profiling with samply

For detailed profiling, use [samply](https://github.com/mstange/samply).

### Installation

```bash
cargo install samply
```

### Usage

```bash
# Build benchmark binary with debug symbols
CARGO_PROFILE_BENCH_DEBUG=true cargo build --release --bench git_stats

# Profile (choose one)
samply record target/release/deps/git_stats-* --bench commits_in_range/7
samply record target/release/deps/git_stats-* --bench commits_in_range/30
samply record target/release/deps/git_stats-* --bench commits_in_range/90
samply record target/release/deps/git_stats-* --bench collect_stats/daily
samply record target/release/deps/git_stats-* --bench collect_stats/weekly
```

This opens Firefox Profiler in your browser with the profiling results.

## Tips

- Use a large repository (1000+ commits) for more accurate benchmarks
- Run benchmarks multiple times to ensure consistent results
- Close other applications to reduce noise
- Use `--warm-up-time` and `--measurement-time` for more samples:

```bash
cargo bench -- --warm-up-time 5 --measurement-time 10
```

## Troubleshooting

### "Gnuplot not found"

This is just a warning. Criterion falls back to the plotters backend automatically.

### Benchmark takes too long

Use a smaller day range or filter to specific benchmarks:

```bash
cargo bench -- commits_in_range/7
```

### Repository not found

Ensure `~/.config/kodo/config.json` exists and contains valid repositories, or set `KODO_BENCH_REPO`:

```bash
KODO_BENCH_REPO=. cargo bench
```
