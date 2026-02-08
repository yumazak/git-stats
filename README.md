# git-stats

[![CI](https://github.com/yumazak/git-stats/actions/workflows/ci.yml/badge.svg)](https://github.com/yumazak/git-stats/actions/workflows/ci.yml)
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

### From Source

```bash
cargo install --path .
```

### From Releases

Download the appropriate binary for your platform from the [Releases](https://github.com/yumazak/git-stats/releases) page.

## Usage

```bash
# Analyze all configured repositories (last 7 days, TUI mode)
git-stats --output tui

# Analyze specific repositories by name
git-stats --repo-name myproject,another-repo --days 7

# JSON output
git-stats --output json --days 30

# CSV output
git-stats --output csv --days 7

# Specify repository path
git-stats --repo ~/projects/my-repo --days 14

# Filter by branch
git-stats --branch main --days 7

# Filter by file extensions
git-stats --ext rs,ts,js --days 7

# Weekly aggregation
git-stats --period weekly --days 30

# Single metric view (default is split view)
git-stats --output tui --single-metric
```

## TUI Controls

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `m` | Toggle view mode (Split/Single) |
| `Tab` / `→` / `l` | Next metric (single view) |
| `Shift+Tab` / `←` / `h` | Previous metric (single view) |
| `↑` / `k` | Scroll up |
| `↓` / `j` | Scroll down |

## Configuration

Create a config file at `~/.config/git-stats/config.json`:

```json
{
  "$schema": "https://raw.githubusercontent.com/yumazak/git-stats/main/schemas/config.schema.json",
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
| `--config` | `-c` | Path to config file | `~/.config/git-stats/config.json` |
| `--repo` | `-r` | Repository path | Current directory |
| `--days` | `-d` | Number of days to analyze | 7 |
| `--output` | `-o` | Output format (tui/json/csv) | json |
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
| `GIT_STATS_CONFIG` | Path to config file |

## License

MIT License - see [LICENSE](LICENSE) for details.
