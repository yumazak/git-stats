# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-02-08

### Added

- `kodo list` subcommand to display registered repositories
- Loading spinner during data collection
- Diverging bar chart for Additions/Deletions (replacing separate charts)
- Vertical scroll support for Add/Del diverging bar chart

### Changed

- Added kodo-release skill for automated releases
- Added release workflow documentation

## [0.3.0] - 2026-02-08

### Added

- `kodo add` subcommand to register repositories to config
- `kodo remove` subcommand to unregister repositories from config

### Changed

- Improved code safety and documented clippy allows
- Removed unused scroll functionality
- Added mise installation method to README

## [0.2.0] - 2026-02-08

### Changed

- **BREAKING:** Renamed crate from `gstat` to `kodo`
- Updated repository URLs after rename

## [0.1.1] - 2026-02-08

### Added

- Initial release with TUI visualization using ratatui
- Git statistics analysis (commits, additions, deletions, net lines, files changed)
- JSON and CSV output formats
- Multi-repository support with config file
- Daily, weekly, monthly, and yearly aggregation periods
- Split view (default) and single metric view modes
- prek pre-commit hooks via mise
- JSON Schema for config validation

### Fixed

- Clippy warnings for Rust 1.93

### Changed

- Default output format to TUI mode
- Removed x86_64-apple-darwin target from release workflow
