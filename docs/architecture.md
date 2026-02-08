# kodo CLI - Architecture Design

## Overview

kodo is a CLI tool that analyzes Git commit history and displays statistics via TUI or exports to JSON/CSV. The architecture follows a layered design with clear separation between CLI handling, domain logic, and infrastructure concerns.

The system is designed for extensibility: new output formats, aggregation periods, and filters can be added without modifying core logic.

---

## 1. Module Structure

```
src/
├── lib.rs                  # Library entry point (re-exports)
├── main.rs                 # CLI entry point (thin wrapper)
│
├── cli/                    # CLI Layer
│   ├── mod.rs
│   ├── args.rs             # clap definitions
│   └── run.rs              # Execution orchestration
│
├── config/                 # Configuration Layer
│   ├── mod.rs
│   ├── schema.rs           # Config structs
│   └── loader.rs           # JSON loading & validation
│
├── git/                    # Git Infrastructure
│   ├── mod.rs
│   ├── repository.rs       # git2 wrapper
│   ├── commit.rs           # Commit data extraction
│   └── diff.rs             # Diff statistics
│
├── stats/                  # Domain Layer
│   ├── mod.rs
│   ├── types.rs            # Core data types
│   ├── collector.rs        # Statistics collection
│   ├── aggregator.rs       # Period aggregation
│   └── filter.rs           # Filtering logic
│
├── output/                 # Output Layer
│   ├── mod.rs
│   ├── json.rs             # JSON formatter
│   ├── csv.rs              # CSV formatter
│   └── format.rs           # Output trait
│
├── tui/                    # TUI Layer
│   ├── mod.rs
│   ├── app.rs              # Application state
│   ├── event.rs            # Event handling
│   ├── ui.rs               # Main UI renderer
│   └── widgets/
│       ├── mod.rs
│       ├── bar_chart.rs    # Bar chart widget
│       └── line_chart.rs   # Line chart widget
│
└── error.rs                # Error types
```

---

## 2. Data Structures

### 2.1 Configuration

```rust
// src/config/schema.rs

use serde::Deserialize;
use std::path::PathBuf;

/// Root configuration structure
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// JSON Schema reference (for IDE support)
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// List of repositories to analyze
    pub repositories: Vec<RepoConfig>,

    /// Default settings
    #[serde(default)]
    pub defaults: Defaults,
}

/// Single repository configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RepoConfig {
    /// Display name
    pub name: String,

    /// Path to repository (supports ~)
    pub path: PathBuf,

    /// Default branch to analyze
    pub branch: Option<String>,
}

/// Default settings
#[derive(Debug, Clone, Deserialize)]
pub struct Defaults {
    /// Number of days to analyze
    #[serde(default = "default_days")]
    pub days: u32,

    /// Exclude merge commits
    #[serde(default = "default_true")]
    pub exclude_merges: bool,
}

fn default_days() -> u32 { 7 }
fn default_true() -> bool { true }

impl Default for Defaults {
    fn default() -> Self {
        Self {
            days: 7,
            exclude_merges: true,
        }
    }
}
```

### 2.2 CLI Arguments

```rust
// src/cli/args.rs

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kodo")]
#[command(version, about = "Analyze Git commit statistics")]
pub struct Args {
    /// Path to config file
    #[arg(short, long, env = "KODO_CONFIG")]
    pub config: Option<PathBuf>,

    /// Repository path (overrides config)
    #[arg(short, long)]
    pub repo: Option<PathBuf>,

    /// Number of days to analyze
    #[arg(short, long, default_value = "7")]
    pub days: u32,

    /// Include merge commits
    #[arg(long)]
    pub include_merges: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "tui")]
    pub output: OutputFormat,

    /// Aggregation period
    #[arg(short, long, value_enum, default_value = "daily")]
    pub period: Period,

    /// Chart type (TUI mode)
    #[arg(long, value_enum, default_value = "bar")]
    pub chart: ChartType,

    /// Branch to analyze
    #[arg(short, long)]
    pub branch: Option<String>,

    /// File extensions to include (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub ext: Option<Vec<String>>,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Tui,
    Json,
    Csv,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Period {
    #[default]
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartType {
    #[default]
    Bar,
    Line,
}
```

### 2.3 Statistics Types

```rust
// src/stats/types.rs

use chrono::NaiveDate;
use serde::Serialize;

/// Statistics for a single time period
#[derive(Debug, Clone, Serialize, Default)]
pub struct PeriodStats {
    /// Period identifier (date, week, month, or year)
    pub label: String,

    /// Start date of the period
    pub date: NaiveDate,

    /// Number of commits
    pub commits: u32,

    /// Lines added
    pub additions: u64,

    /// Lines deleted
    pub deletions: u64,

    /// Number of files changed
    pub files_changed: u32,
}

impl PeriodStats {
    /// Calculate net line change
    pub fn net_lines(&self) -> i64 {
        self.additions as i64 - self.deletions as i64
    }

    /// Merge another period's stats into this one
    pub fn merge(&mut self, other: &Self) {
        self.commits += other.commits;
        self.additions += other.additions;
        self.deletions += other.deletions;
        self.files_changed += other.files_changed;
    }
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    /// Repository name
    pub repository: String,

    /// Aggregation period type
    pub period: String,

    /// Start date of analysis
    pub from: NaiveDate,

    /// End date of analysis
    pub to: NaiveDate,

    /// Statistics per period
    pub stats: Vec<PeriodStats>,

    /// Total statistics
    pub total: TotalStats,
}

/// Aggregated total statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct TotalStats {
    pub commits: u32,
    pub additions: u64,
    pub deletions: u64,
    pub net_lines: i64,
    pub files_changed: u32,
}

impl TotalStats {
    /// Calculate from period stats
    pub fn from_periods(periods: &[PeriodStats]) -> Self {
        let mut total = Self::default();
        for p in periods {
            total.commits += p.commits;
            total.additions += p.additions;
            total.deletions += p.deletions;
            total.files_changed += p.files_changed;
        }
        total.net_lines = total.additions as i64 - total.deletions as i64;
        total
    }
}
```

### 2.4 Git Types

```rust
// src/git/commit.rs

use chrono::{DateTime, Utc};

/// Extracted commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    /// Commit hash (short)
    pub id: String,

    /// Commit timestamp (UTC)
    pub timestamp: DateTime<Utc>,

    /// Is this a merge commit?
    pub is_merge: bool,

    /// Diff statistics
    pub diff: DiffStats,
}

/// Diff statistics for a commit
#[derive(Debug, Clone, Default)]
pub struct DiffStats {
    pub additions: u64,
    pub deletions: u64,
    pub files_changed: u32,
    pub files: Vec<FileChange>,
}

/// Individual file change
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub additions: u64,
    pub deletions: u64,
}
```

### 2.5 Error Types

```rust
// src/error.rs

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    ConfigInvalid { message: String },

    #[error("Repository not found: {path}")]
    RepoNotFound { path: PathBuf },

    #[error("Not a git repository: {path}")]
    NotGitRepo { path: PathBuf },

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("No repositories to analyze")]
    NoRepositories,
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
```

---

## 3. Module Responsibilities & Public APIs

### 3.1 CLI Module (`cli/`)

**Responsibility:** Parse arguments, orchestrate execution, handle errors

```rust
// src/cli/mod.rs
pub mod args;
pub mod run;

pub use args::Args;
pub use run::execute;
```

```rust
// src/cli/run.rs
use crate::cli::Args;
use crate::error::Result;

/// Main execution entry point
pub fn execute(args: Args) -> Result<()>;
```

### 3.2 Config Module (`config/`)

**Responsibility:** Load and validate configuration

```rust
// src/config/mod.rs
pub mod loader;
pub mod schema;

pub use loader::load_config;
pub use schema::{Config, Defaults, RepoConfig};
```

```rust
// src/config/loader.rs
use crate::config::Config;
use crate::error::Result;
use std::path::Path;

/// Load configuration from file
pub fn load_config(path: &Path) -> Result<Config>;

/// Get default config path (~/.config/kodo/config.json)
pub fn default_config_path() -> Option<PathBuf>;

/// Expand ~ in path
pub fn expand_tilde(path: &Path) -> PathBuf;
```

### 3.3 Git Module (`git/`)

**Responsibility:** Interface with git repositories

```rust
// src/git/mod.rs
pub mod commit;
pub mod diff;
pub mod repository;

pub use commit::CommitInfo;
pub use diff::DiffStats;
pub use repository::Repository;
```

```rust
// src/git/repository.rs
use crate::error::Result;
use crate::git::CommitInfo;
use chrono::NaiveDate;
use std::path::Path;

pub struct Repository {
    inner: git2::Repository,
    name: String,
}

impl Repository {
    /// Open a repository
    pub fn open(path: &Path, name: &str) -> Result<Self>;

    /// Get commits in date range
    pub fn commits_in_range(
        &self,
        from: NaiveDate,
        to: NaiveDate,
        branch: Option<&str>,
        exclude_merges: bool,
    ) -> Result<Vec<CommitInfo>>;
}
```

### 3.4 Stats Module (`stats/`)

**Responsibility:** Aggregate and filter statistics

```rust
// src/stats/mod.rs
pub mod aggregator;
pub mod collector;
pub mod filter;
pub mod types;

pub use aggregator::aggregate;
pub use collector::collect_stats;
pub use filter::Filter;
pub use types::{AnalysisResult, PeriodStats, TotalStats};
```

```rust
// src/stats/collector.rs
use crate::cli::args::Period;
use crate::git::CommitInfo;
use crate::stats::{AnalysisResult, Filter};
use chrono::NaiveDate;

/// Collect statistics from commits
pub fn collect_stats(
    repo_name: &str,
    commits: Vec<CommitInfo>,
    from: NaiveDate,
    to: NaiveDate,
    period: Period,
    filter: Option<&Filter>,
) -> AnalysisResult;
```

```rust
// src/stats/aggregator.rs
use crate::cli::args::Period;
use crate::stats::PeriodStats;
use chrono::NaiveDate;

/// Aggregate stats by period
pub fn aggregate(
    stats: Vec<PeriodStats>,
    period: Period,
    from: NaiveDate,
    to: NaiveDate,
) -> Vec<PeriodStats>;
```

### 3.5 Output Module (`output/`)

**Responsibility:** Format and output results

```rust
// src/output/mod.rs
pub mod csv;
pub mod format;
pub mod json;

pub use format::Formatter;
```

```rust
// src/output/format.rs
use crate::error::Result;
use crate::stats::AnalysisResult;

/// Output formatter trait
pub trait Formatter {
    fn format(&self, result: &AnalysisResult) -> Result<String>;
}
```

```rust
// src/output/json.rs
pub struct JsonFormatter;
impl Formatter for JsonFormatter { ... }
```

```rust
// src/output/csv.rs
pub struct CsvFormatter;
impl Formatter for CsvFormatter { ... }
```

### 3.6 TUI Module (`tui/`)

**Responsibility:** Terminal UI rendering and interaction

```rust
// src/tui/mod.rs
pub mod app;
pub mod event;
pub mod ui;
pub mod widgets;

pub use app::App;
```

```rust
// src/tui/app.rs
use crate::cli::args::ChartType;
use crate::error::Result;
use crate::stats::AnalysisResult;

pub struct App {
    result: AnalysisResult,
    chart_type: ChartType,
    selected_metric: Metric,
    should_quit: bool,
}

impl App {
    pub fn new(result: AnalysisResult, chart_type: ChartType) -> Self;
    pub fn run(&mut self) -> Result<()>;
}

#[derive(Clone, Copy)]
pub enum Metric {
    Commits,
    Additions,
    Deletions,
    NetLines,
    FilesChanged,
}
```

---

## 4. Error Handling Strategy

### 4.1 Layer-specific Approach

| Layer | Strategy |
|-------|----------|
| `main.rs` | Catch all errors, display user-friendly message, exit with code |
| `cli/` | Use `anyhow::Result` for context, convert to `Error` at boundary |
| Domain/Infra | Use custom `Error` enum with `thiserror` |
| TUI | Recover gracefully, show error in UI when possible |

### 4.2 Error Display

```rust
// src/main.rs
fn main() {
    if let Err(e) = kodo::cli::execute(Args::parse()) {
        eprintln!("error: {e}");

        // Print cause chain
        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {cause}");
            source = cause.source();
        }

        std::process::exit(1);
    }
}
```

### 4.3 Recoverable vs Unrecoverable

| Error Type | Handling |
|------------|----------|
| Config not found | Exit with message |
| Repo not found | Exit with message |
| Git error (permission) | Exit with message |
| Empty repo | Continue with empty stats |
| No commits in range | Continue with zero stats |

---

## 5. Task Breakdown with Dependencies

```
┌─────────────────────────────────────────────────────────────────┐
│                         MVP PHASE                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [T1] Project Setup ──────┐                                    │
│    • Cargo.toml            │                                    │
│    • rust-toolchain.toml   │                                    │
│    • .gitignore            │                                    │
│                            ▼                                    │
│  [T2] Error Types ◄───── [T1]                                  │
│    • src/error.rs                                               │
│                            │                                    │
│           ┌────────────────┼────────────────┐                   │
│           ▼                ▼                ▼                   │
│  [T3] Config        [T4] CLI Args    [T5] Git Module           │
│    • schema.rs        • args.rs        • repository.rs          │
│    • loader.rs                         • commit.rs              │
│                                        • diff.rs                │
│           │                │                │                   │
│           └────────────────┼────────────────┘                   │
│                            ▼                                    │
│                    [T6] Stats Types                             │
│                      • types.rs                                 │
│                            │                                    │
│           ┌────────────────┴────────────────┐                   │
│           ▼                                 ▼                   │
│  [T7] Collector                     [T8] JSON Output           │
│    • collector.rs                     • json.rs                 │
│    • aggregator.rs                    • format.rs               │
│           │                                 │                   │
│           └────────────────┬────────────────┘                   │
│                            ▼                                    │
│                    [T9] CLI Runner                              │
│                      • run.rs                                   │
│                            │                                    │
│                            ▼                                    │
│                    [T10] Integration Test                       │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                         TUI PHASE                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [T11] TUI App State ◄── [T9]                                  │
│    • app.rs                                                     │
│           │                                                     │
│           ├──────────────────┐                                  │
│           ▼                  ▼                                  │
│  [T12] Bar Chart     [T13] Event Handler                       │
│    • bar_chart.rs      • event.rs                               │
│           │                  │                                  │
│           └────────┬─────────┘                                  │
│                    ▼                                            │
│            [T14] UI Renderer                                    │
│              • ui.rs                                            │
│                    │                                            │
│                    ▼                                            │
│            [T15] Line Chart                                     │
│              • line_chart.rs                                    │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                      EXTENSION PHASE                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [T16] CSV Output ◄── [T8]                                     │
│  [T17] Period Aggregation ◄── [T7]                             │
│  [T18] Branch Filter ◄── [T5]                                  │
│  [T19] Extension Filter ◄── [T5]                               │
│  [T20] Multi-repo ◄── [T9]                                     │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                      QUALITY PHASE                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  [T21] Unit Tests (parallel with implementation)               │
│  [T22] README.md + README.ja.md                                │
│  [T23] GitHub Actions CI                                       │
│  [T24] GitHub Actions Release                                  │
│  [T25] prek setup                                              │
│  [T26] JSON Schema                                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Task Estimates

| Task | Effort | Dependencies |
|------|--------|--------------|
| T1: Project Setup | 30min | - |
| T2: Error Types | 30min | T1 |
| T3: Config Module | 1h | T2 |
| T4: CLI Args | 1h | T2 |
| T5: Git Module | 2h | T2 |
| T6: Stats Types | 30min | T3, T4, T5 |
| T7: Collector | 1.5h | T6 |
| T8: JSON Output | 30min | T6 |
| T9: CLI Runner | 1h | T7, T8 |
| T10: Integration Test | 1h | T9 |
| T11-T15: TUI | 4h | T9 |
| T16-T20: Extensions | 4h | varies |
| T21-T26: Quality | 4h | T10 |

---

## 6. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| git2 API complexity | High | Medium | Wrap in simple Repository struct, extensive tests |
| Large repo performance | Medium | Low | Use git2 revwalk efficiently, consider parallel processing |
| ratatui breaking changes | Medium | Low | Pin version, test TUI separately |
| Cross-platform terminal issues | Medium | Medium | Test on macOS, Linux, Windows in CI |
| Binary file handling | Low | Medium | Skip binary files (git default behavior) |

---

## 7. Test Hooks

### Unit Tests

| Module | Test Focus |
|--------|------------|
| `config/loader` | JSON parsing, ~ expansion, validation errors |
| `git/repository` | Mock git2 where possible, use tempfile for real tests |
| `stats/collector` | Period grouping, zero-fill, edge cases |
| `stats/aggregator` | Weekly/monthly grouping, date boundaries |
| `output/json` | JSON schema conformance |
| `output/csv` | Header, escaping, encoding |

### Integration Tests

| Test | Description |
|------|-------------|
| `cli_json_output` | Full CLI → JSON output with temp repo |
| `cli_help` | --help displays correctly |
| `cli_invalid_repo` | Error message for non-existent path |
| `tui_smoke` | TUI starts and quits without crash |

### Test Fixtures

```
tests/fixtures/
├── config_valid.json       # Valid config
├── config_invalid.json     # Missing required fields
├── config_schema.json      # With $schema
└── repos/                  # Created at test time via tempfile
```

---

## Decisions & Assumptions

| Decision | Rationale |
|----------|-----------|
| Single binary (lib + bin) | Simpler distribution, lib for testing |
| git2 over subprocess | Native Rust, better error handling, no PATH dependency |
| Metric enum in TUI | Type-safe metric switching |
| UTC everywhere | Avoid timezone bugs |
| Include zero-stat days | Better visualization continuity |
| Default branch = HEAD | Simple default, --branch overrides |

---

## 8. Design Review (Rust Skills)

### 8.1 domain-cli Review

| Check | Status | Notes |
|-------|--------|-------|
| clap derive macros | ✅ | `#[derive(Parser)]` used |
| CLI > env > config priority | ✅ | `env = "KODO_CONFIG"` in Args |
| Errors to stderr | ✅ | `eprintln!` in main.rs |
| Non-zero exit on error | ✅ | `std::process::exit(1)` |
| Progress for long ops | ⚠️ ADD | Need `indicatif` for git analysis |
| Ctrl+C handling | ⚠️ ADD | Need signal handler in TUI |

**Action Items:**
- Add `indicatif = "0.17"` to Cargo.toml
- Add progress bar in `git/repository.rs` for large repos
- Add Ctrl+C handler in `tui/event.rs`

### 8.2 m06-error-handling Review

| Check | Status | Notes |
|-------|--------|-------|
| thiserror for library | ✅ | `Error` enum with thiserror |
| anyhow for app layer | ✅ | Used in `cli/run.rs` |
| `.context()` usage | ⚠️ ADD | Need explicit context in error propagation |
| Error chain display | ✅ | Implemented in main.rs |
| Recoverable vs panic | ✅ | Clear separation defined |

**Action Items:**
- Use `.context("what was happening")` pattern throughout
- Example: `fs::read_to_string(path).context("reading config file")?`

### 8.3 m05-type-driven Review

| Check | Status | Notes |
|-------|--------|-------|
| Newtype for primitives | ⚠️ ADD | Missing type safety |
| Type state pattern | N/A | Not needed for this scope |
| Enum for states | ✅ | `Metric`, `OutputFormat`, `Period` enums |
| Builder pattern | N/A | Simple structs, not needed |

**Action Items - Add Newtypes:**

```rust
// src/stats/types.rs - Add newtypes for type safety

/// Days count (non-negative)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Days(pub u32);

impl Days {
    pub fn new(days: u32) -> Self {
        Self(days)
    }
}

/// Date range for analysis
#[derive(Debug, Clone, Copy)]
pub struct DateRange {
    pub from: NaiveDate,
    pub to: NaiveDate,
}

impl DateRange {
    pub fn last_n_days(days: Days) -> Self {
        let to = Utc::now().date_naive();
        let from = to - chrono::Duration::days(days.0 as i64);
        Self { from, to }
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.from && date <= self.to
    }
}

/// Repository path (validated)
#[derive(Debug, Clone)]
pub struct RepoPath(PathBuf);

impl RepoPath {
    pub fn new(path: PathBuf) -> Result<Self> {
        let expanded = expand_tilde(&path);
        if !expanded.join(".git").exists() {
            return Err(Error::NotGitRepo { path: expanded });
        }
        Ok(Self(expanded))
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}
```

### 8.4 Updated Module Structure

```
src/
├── lib.rs
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs
│   ├── run.rs
│   └── progress.rs          # NEW: Progress bar wrapper
├── config/
│   └── ...
├── git/
│   └── ...
├── stats/
│   ├── mod.rs
│   ├── types.rs              # Updated with newtypes
│   ├── date_range.rs         # NEW: DateRange type
│   └── ...
├── output/
│   └── ...
├── tui/
│   ├── mod.rs
│   ├── app.rs
│   ├── event.rs              # Updated with Ctrl+C
│   └── ...
└── error.rs
```

### 8.5 Updated Cargo.toml Dependencies

```toml
[dependencies]
# ... existing deps ...

# Progress display (domain-cli recommendation)
indicatif = "0.17"

# Better context for errors (m06-error-handling)
# Already using anyhow, ensure .context() is used

# Signal handling for TUI
ctrlc = "3.4"
```

---

## Questions

None - proceeding with documented decisions.
