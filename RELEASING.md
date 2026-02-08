# Releasing kodo

This document describes the release process for kodo.

## Release Checklist

- [ ] Ensure all changes are merged to `main`
- [ ] Ensure CI is passing on `main`
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` (move Unreleased items to new version section)
- [ ] Commit changes: `git commit -m "chore: Bump version to X.Y.Z"`
- [ ] Create and push tag: `git tag vX.Y.Z && git push origin main --tags`
- [ ] Verify GitHub Actions release workflow completes successfully
- [ ] Verify crates.io publication

## Version Guidelines

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (X.0.0): Breaking changes that require user action
- **MINOR** (0.X.0): New features, backwards compatible
- **PATCH** (0.0.X): Bug fixes, backwards compatible

### Examples

- Breaking CLI option change → MAJOR
- New subcommand added → MINOR
- Bug fix in statistics calculation → PATCH

## GitHub Actions Workflow

The release workflow (`.github/workflows/release.yml`) is triggered by pushing a tag matching `v*`:

1. **Build**: Creates release binaries for supported platforms
2. **Release**: Creates GitHub Release with auto-generated release notes
3. **Publish**: Publishes to crates.io

## GitHub Branch Protection Setup

To protect the `main` branch:

1. Go to repository Settings → Branches
2. Click "Add branch ruleset" or "Add rule"
3. Set branch name pattern: `main`
4. Enable the following:
   - Require a pull request before merging
   - Require status checks to pass before merging
   - Select required status checks (CI workflow)
5. Save changes

## Troubleshooting

### crates.io publish fails

- Ensure `CARGO_REGISTRY_TOKEN` secret is set in repository settings
- Verify the token has publish permissions

### Release workflow fails

- Check if the tag matches the version in `Cargo.toml`
- Ensure the tag format is `vX.Y.Z` (e.g., `v0.3.0`)
