# kodo

[![CI](https://github.com/yumazak/kodo/actions/workflows/ci.yml/badge.svg)](https://github.com/yumazak/kodo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A CLI tool for analyzing Git commit statistics with TUI visualization.

[日本語](README.ja.md)

## Features

- Analyze commit history with date range filtering
- Interactive TUI with bar and line charts
- Split view showing all metrics simultaneously
- Export data in JSON or CSV format
- Filter by branch and file extensions
- Daily, weekly, monthly, and yearly aggregation
- Multi-repository configuration support

## Installation

### mise (Recommended)

```bash
mise use -g github:yumazak/kodo
```

### crates.io

```bash
cargo install kodo
```

### From Releases

Download the appropriate binary for your platform from the [Releases](https://github.com/yumazak/kodo/releases) page.

### From Source

```bash
cargo install --path .
```

## Usage

```bash
# Analyze all configured repositories (last 7 days, TUI mode)
kodo

# TUI output with short option
kodo -o tui

# Analyze specific repositories by name
kodo --repo-name myproject,another-repo --days 7

# JSON output
kodo --output json --days 30

# CSV output
kodo --output csv --days 7

# Specify repository path
kodo --repo ~/projects/my-repo --days 14

# Filter by branch
kodo --branch main --days 7

# Filter by file extensions
kodo --ext rs,ts,js --days 7

# Weekly aggregation
kodo --period weekly --days 30

# Single metric view (default is split view)
kodo --single-metric
```

## TUI Controls

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `m` | Toggle view mode (Split/Single) |
| `Tab` / `→` / `l` | Next metric (single view) |
| `Shift+Tab` / `←` / `h` | Previous metric (single view) |

## Configuration

Create a config file at `~/.config/kodo/config.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/yumazak/kodo/main/schemas/config.schema.json",
  "repositories": [
    {
      "name": "my-project",
      "path": "~/projects/my-project",
      "branch": "main"
    },
    {
      "name": "another-repo",
      "path": "~/work/another-repo"
    }
  ],
  "defaults": {
    "days": 7,
    "exclude_merges": true
  }
}
```

## CLI Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--config` | `-c` | Path to config file | `~/.config/kodo/config.json` |
| `--repo` | `-r` | Repository path | Current directory |
| `--days` | `-d` | Number of days to analyze | 7 |
| `--output` | `-o` | Output format (table/tui/json/csv) | table |
| `--period` | `-p` | Aggregation period (daily/weekly/monthly/yearly) | daily |
| `--branch` | `-b` | Branch to analyze | Default branch |
| `--ext` | | File extensions to include (comma-separated) | All files |
| `--include-merges` | | Include merge commits | false |
| `--single-metric` | | Show single metric in TUI | false (split view) |
| `--repo-name` | | Filter repositories by name (comma-separated) | All repos |

## Metrics

- **Commits**: Number of commits
- **Additions**: Lines added
- **Deletions**: Lines deleted
- **Net Lines**: Additions - Deletions (can be negative)
- **Files Changed**: Number of files modified

## Environment Variables

| Variable | Description |
|----------|-------------|
| `KODO_CONFIG` | Path to config file |

## License

MIT License - see [LICENSE](LICENSE) for details.
