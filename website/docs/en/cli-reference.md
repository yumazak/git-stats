# CLI Reference

## Main Command

```bash
kodo [OPTIONS]
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--config` | `-c` | Path to config file | `~/.config/kodo/config.json` |
| `--repo` | `-r` | Repository path | Current directory |
| `--days` | `-d` | Number of days to analyze | 7 |
| `--output` | `-o` | Output format (tui/json/csv) | tui |
| `--period` | `-p` | Aggregation period (daily/weekly/monthly/yearly) | daily |
| `--branch` | `-b` | Branch to analyze | Default branch |
| `--ext` | | File extensions to include (comma-separated) | All files |
| `--include-merges` | | Include merge commits | false |
| `--single-metric` | | Show single metric in TUI | false (split view) |
| `--repo-name` | | Filter repositories by name (comma-separated) | All repos |

## Subcommands

### `kodo add <path>`

Add a repository to the configuration.

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `<path>` | | Path to the repository (use `.` for current directory) | Required |
| `--name` | `-n` | Display name for the repository | Directory name |
| `--branch` | `-b` | Default branch to analyze | None |

**Examples:**

```bash
kodo add .
kodo add . --name my-project
kodo add /path/to/repo --name myrepo --branch develop
```

### `kodo remove <identifier>`

Remove a repository from the configuration.

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `<identifier>` | | Repository path or name to remove | Required |

**Examples:**

```bash
kodo remove .
kodo remove my-project
kodo remove /path/to/repo
```

### `kodo list`

List registered repositories.

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--json` | | Output in JSON format | false (table format) |

**Examples:**

```bash
kodo list
kodo list --json
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `KODO_CONFIG` | Path to config file |
