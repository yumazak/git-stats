//! Benchmark for git statistics collection
//!
//! Run with: `cargo bench`
//!
//! Repository selection (in order of priority):
//! 1. First repository from ~/.config/kodo/config.json
//! 2. Environment variable `KODO_BENCH_REPO`
//! 3. Current directory (.)

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use kodo::cli::args::Period;
use kodo::config::{default_config_path, expand_tilde, load_config};
use kodo::git::Repository;
use kodo::stats::{DateRange, Days, collect_stats};
use std::env;
use std::path::PathBuf;

/// Configuration for benchmarks
struct BenchmarkConfig {
    repo_path: PathBuf,
    repo_name: String,
}

impl BenchmarkConfig {
    /// Load benchmark configuration
    ///
    /// Priority:
    /// 1. First repository from ~/.config/kodo/config.json
    /// 2. Environment variable `KODO_BENCH_REPO`
    /// 3. Current directory (.)
    fn load() -> Self {
        // Try config.json first
        if let Some(config_path) = default_config_path()
            && let Ok(config) = load_config(&config_path)
            && let Some(repo) = config.repositories.first()
        {
            let repo_path = expand_tilde(&repo.path);
            return Self {
                repo_path,
                repo_name: repo.name.clone(),
            };
        }

        // Try environment variable
        if let Ok(path) = env::var("KODO_BENCH_REPO") {
            let repo_path = expand_tilde(&PathBuf::from(&path));
            let repo_name = repo_path.file_name().map_or_else(
                || "benchmark-repo".to_string(),
                |n| n.to_string_lossy().to_string(),
            );
            return Self {
                repo_path,
                repo_name,
            };
        }

        // Fallback to current directory
        Self {
            repo_path: PathBuf::from("."),
            repo_name: "current-dir".to_string(),
        }
    }
}

/// Benchmark `Repository::commits_in_range` with different day ranges
fn bench_commits_in_range(c: &mut Criterion) {
    let config = BenchmarkConfig::load();

    let repo = Repository::open(&config.repo_path, &config.repo_name).unwrap_or_else(|e| {
        panic!(
            "Failed to open repository at {}: {}",
            config.repo_path.display(),
            e
        )
    });

    println!(
        "Benchmarking repository: {} ({})",
        config.repo_name,
        config.repo_path.display()
    );

    let mut group = c.benchmark_group("commits_in_range");

    for days in [7, 30, 90] {
        group.bench_with_input(BenchmarkId::new("days", days), &days, |b, &days| {
            let range = DateRange::last_n_days(Days::new(days));
            b.iter(|| {
                repo.commits_in_range(
                    black_box(range.from),
                    black_box(range.to),
                    None,
                    true, // exclude merges
                )
            });
        });
    }

    group.finish();
}

/// Benchmark `collect_stats` function
fn bench_collect_stats(c: &mut Criterion) {
    let config = BenchmarkConfig::load();

    let repo = Repository::open(&config.repo_path, &config.repo_name).unwrap_or_else(|e| {
        panic!(
            "Failed to open repository at {}: {}",
            config.repo_path.display(),
            e
        )
    });

    // Pre-fetch commits for 30 days
    let range = DateRange::last_n_days(Days::new(30));
    let commits = repo
        .commits_in_range(range.from, range.to, None, true)
        .expect("Failed to fetch commits");

    println!("Benchmarking collect_stats with {} commits", commits.len());

    let mut group = c.benchmark_group("collect_stats");

    group.bench_function("daily", |b| {
        b.iter(|| {
            collect_stats(
                black_box(&config.repo_name),
                black_box(commits.clone()),
                black_box(range),
                black_box(Period::Daily),
                None,
            )
        });
    });

    group.bench_function("weekly", |b| {
        b.iter(|| {
            collect_stats(
                black_box(&config.repo_name),
                black_box(commits.clone()),
                black_box(range),
                black_box(Period::Weekly),
                None,
            )
        });
    });

    group.finish();
}

criterion_group!(benches, bench_commits_in_range, bench_collect_stats);
criterion_main!(benches);
