# Configuration

## Config File Location

The default configuration file is located at `~/.config/kodo/config.json`.

You can specify a different path using:
- The `--config` option: `kodo --config /path/to/config.json`
- The `KODO_CONFIG` environment variable

## Config File Structure

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

## Repository Configuration

Each repository in the `repositories` array can have the following fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Display name for the repository |
| `path` | string | Yes | Path to the repository (supports `~` expansion) |
| `branch` | string | No | Default branch to analyze |

## Default Settings

The `defaults` object configures default behavior:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `days` | number | 7 | Number of days to analyze |
| `exclude_merges` | boolean | true | Exclude merge commits from analysis |

## JSON Schema

The config file supports JSON Schema validation. Add the `$schema` field to enable editor autocompletion and validation:

```json
{
  "$schema": "https://raw.githubusercontent.com/yumazak/kodo/main/schemas/config.schema.json"
}
```

## Managing Repositories

Instead of manually editing the config file, you can use CLI commands:

```bash
# Add a repository
kodo add /path/to/repo --name my-repo

# Remove a repository
kodo remove my-repo

# List all repositories
kodo list
```
