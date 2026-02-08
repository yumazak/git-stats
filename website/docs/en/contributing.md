# Contributing

Thank you for your interest in contributing to kodo!

## Getting Started

### Prerequisites

- Rust 1.93 or later
- [mise](https://mise.jdx.dev/) for toolchain management (recommended)

### Building from Source

```bash
git clone https://github.com/yumazak/kodo.git
cd kodo
cargo build
```

### Running Tests

```bash
cargo test
```

## Development Workflow

### Branch Naming

We use [GitHub Flow](https://docs.github.com/en/get-started/using-github/github-flow):

- `feature/*` - New features
- `fix/*` - Bug fixes
- `docs/*` - Documentation updates
- `refactor/*` - Code refactoring
- `chore/*` - Maintenance tasks

### Commit Messages

We recommend [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: Add new feature
fix: Fix bug in statistics calculation
docs: Update README
refactor: Improve error handling
chore: Update dependencies
```

## Pull Request Process

1. Create a branch from `main`
2. Make your changes
3. Ensure all tests pass: `cargo test`
4. Ensure code is formatted: `cargo fmt`
5. Ensure no clippy warnings: `cargo clippy`
6. Open a pull request to `main`
7. Wait for CI to pass and code review

## Code Style

### Formatting and Linting

This project uses:

- `cargo fmt` for code formatting
- `cargo clippy` for linting
- [prek](https://github.com/j178/prek) for pre-commit hooks (via mise)

Pre-commit hooks are automatically set up when you install mise:

```bash
mise install
```

### Guidelines

- Follow Rust naming conventions
- Write documentation for public APIs
- Add tests for new functionality
- Keep functions focused and small

## Branch Protection

The `main` branch is protected:

- All changes require a pull request
- CI must pass before merging
- Direct pushes to `main` are not allowed

## Questions?

Feel free to open an issue if you have questions or need help.
