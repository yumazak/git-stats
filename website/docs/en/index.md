# kodo

A CLI tool for analyzing Git commit statistics with TUI visualization.

## Features

- Analyze commit history with date range filtering
- Interactive TUI with bar and line charts
- Split view showing all metrics simultaneously
- Export data in JSON or CSV format
- Filter by branch and file extensions
- Daily, weekly, monthly, and yearly aggregation
- Multi-repository configuration support

## Quick Start

```bash
# Install via mise (recommended)
mise use -g github:yumazak/kodo

# Or via cargo
cargo install kodo

# Run analysis
kodo
```

## Metrics

- **Commits**: Number of commits
- **Additions**: Lines added
- **Deletions**: Lines deleted
- **Net Lines**: Additions - Deletions (can be negative)
- **Files Changed**: Number of files modified

## License

MIT License - see [LICENSE](https://github.com/yumazak/kodo/blob/main/LICENSE) for details.
