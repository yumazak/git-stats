# git-stats CLI - Requirements Specification

## Goals

1. Analyze Git commit history across multiple repositories
2. Display statistics via TUI (bar chart, line chart)
3. Export data in JSON/CSV format
4. Provide filtering by date range, branch, and file types

## Non-Goals

- GitHub PR count integration (explicitly excluded per spec)
- Real-time monitoring / watch mode
- Web UI or GUI
- Git operations (commit, push, etc.)
- Repository management (clone, init)

---

## Scope Definition

### MVP (Phase 1)

| ID | Feature | Priority |
|----|---------|----------|
| F1 | JSON config loading with `$schema` support | P0 |
| F2 | Single repository analysis via CLI arg | P0 |
| F3 | Date range filtering (default: 7 days) | P0 |
| F4 | Merge commit exclusion (default: on) | P0 |
| F5 | Collect: commits, additions, deletions, net lines, files changed | P0 |
| F6 | JSON output | P0 |
| F7 | Daily aggregation | P0 |

### Phase 2: TUI

| ID | Feature | Priority |
|----|---------|----------|
| F8 | Bar chart display (ratatui) | P0 |
| F9 | Line chart display (ratatui) | P1 |
| F10 | Keyboard navigation (q: quit, Tab: switch) | P0 |
| F11 | Multi-metric view toggle | P1 |

### Phase 3: Extended Features

| ID | Feature | Priority |
|----|---------|----------|
| F12 | CSV output | P1 |
| F13 | Weekly/Monthly/Yearly aggregation | P1 |
| F14 | Branch filtering | P2 |
| F15 | File extension filtering | P2 |
| F16 | Multi-repository analysis | P2 |

---

## Acceptance Criteria

### F1: JSON Config Loading

- [ ] Load config from `~/.config/git-stats/config.json` by default
- [ ] Support `--config` flag for custom path
- [ ] Support `$schema` field for IDE validation
- [ ] Validate required fields: `repositories[].name`, `repositories[].path`
- [ ] Expand `~` to home directory in paths
- [ ] Error with clear message if config file not found
- [ ] Error if repository path does not exist or is not a git repo

### F2: Single Repository Analysis

- [ ] Accept `--repo <path>` to analyze a single repository
- [ ] `--repo` overrides config file repositories
- [ ] Validate path is a git repository (has `.git` directory)

### F3: Date Range Filtering

- [ ] Accept `--days <N>` (default: 7)
- [ ] Calculate date range as `[today - N days, today]`
- [ ] Use UTC timezone for consistency
- [ ] Accept `--from` and `--to` for explicit date range (optional enhancement)

### F4: Merge Commit Exclusion

- [ ] Exclude merge commits by default
- [ ] Accept `--include-merges` flag to include them
- [ ] Identify merge commits by having 2+ parents

### F5: Statistics Collection

For each period, collect:
- [ ] `commits`: Number of commits
- [ ] `additions`: Lines added
- [ ] `deletions`: Lines deleted
- [ ] `net_lines`: additions - deletions
- [ ] `files_changed`: Number of unique files modified

### F6: JSON Output

- [ ] Accept `--output json` flag
- [ ] Output valid JSON to stdout
- [ ] Schema:
```json
{
  "repository": "repo-name",
  "period": "daily",
  "from": "2024-01-01",
  "to": "2024-01-07",
  "stats": [
    {
      "date": "2024-01-01",
      "commits": 5,
      "additions": 100,
      "deletions": 20,
      "net_lines": 80,
      "files_changed": 10
    }
  ],
  "total": {
    "commits": 25,
    "additions": 500,
    "deletions": 100,
    "net_lines": 400,
    "files_changed": 50
  }
}
```

### F7: Daily Aggregation

- [ ] Group commits by date (UTC)
- [ ] Sum statistics per day
- [ ] Include days with zero commits in output

### F8: Bar Chart Display

- [ ] Default output mode when terminal is interactive
- [ ] Display commits per day as horizontal bars
- [ ] Show date labels on Y-axis
- [ ] Show value labels on bars
- [ ] Respect terminal width for scaling

### F9: Line Chart Display

- [ ] Accept `--chart line` flag
- [ ] Display trends over time
- [ ] Support multiple metrics overlay (optional)

### F10: Keyboard Navigation

- [ ] `q` or `Esc`: Quit application
- [ ] `Tab`: Switch between metrics
- [ ] Arrow keys: Scroll if data exceeds screen

### F12: CSV Output

- [ ] Accept `--output csv` flag
- [ ] Output valid CSV with header row
- [ ] Columns: date,commits,additions,deletions,net_lines,files_changed

### F13: Period Aggregation

- [ ] Accept `--period daily|weekly|monthly|yearly`
- [ ] Weekly: Group by ISO week
- [ ] Monthly: Group by year-month
- [ ] Yearly: Group by year

### F14: Branch Filtering

- [ ] Accept `--branch <name>` flag
- [ ] Analyze only commits reachable from specified branch
- [ ] Default: current branch

### F15: File Extension Filtering

- [ ] Accept `--ext <extensions>` (comma-separated)
- [ ] Example: `--ext rs,ts,js`
- [ ] Only count changes to files with matching extensions

---

## Technical Constraints

| Constraint | Details |
|------------|---------|
| Rust Version | 1.93 (fixed via rust-toolchain.toml) |
| Edition | 2024 |
| TUI Framework | ratatui 0.30.0+ |
| Git Library | git2 (libgit2 bindings) |
| License | MIT |
| CI/CD | GitHub Actions |
| Pre-commit | prek (cargo fmt, clippy) |
| Documentation | English (README.md, code comments) |
| README | English (main) + Japanese (README.ja.md) |

---

## Decisions & Assumptions

| Decision | Rationale |
|----------|-----------|
| JSON only (no YAML) | $schema support is cleaner with JSON |
| UTC for all dates | Consistency across timezones |
| git2 over shell commands | Native Rust, better performance, no external dependencies |
| No PR count | Explicitly excluded in spec |
| Default 7 days | Matches existing shell script behavior |
| Merge commits excluded by default | Focus on actual work, not merges |

---

## Edge Cases

| Case | Handling |
|------|----------|
| Empty repository | Return empty stats array, no error |
| No commits in date range | Return stats with all zeros |
| Invalid config path | Error with clear message |
| Non-git directory | Error with clear message |
| Binary files | Excluded from line counts (git default) |
| Renamed files | Count as 1 file changed |
| Permission denied | Error with path info |

---

## Questions

None - proceeding with documented decisions.
