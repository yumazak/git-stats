# Getting Started

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

## Basic Usage

```bash
# Analyze all configured repositories (last 7 days, TUI mode)
kodo

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
| `↑` / `k` | Scroll up |
| `↓` / `j` | Scroll down |

## Repository Management

### Add a repository

```bash
kodo add .
kodo add . --name my-project
kodo add /path/to/repo --name myrepo --branch develop
```

### Remove a repository

```bash
kodo remove .
kodo remove my-project
kodo remove /path/to/repo
```

### List repositories

```bash
kodo list
kodo list --json
```
