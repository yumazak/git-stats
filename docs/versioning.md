# Versioning Guide

## Semantic Versioning

kodo follows [Semantic Versioning 2.0.0](https://semver.org/).

### Version Format: MAJOR.MINOR.PATCH

| Type | Example | Description |
|------|---------|-------------|
| MAJOR | 1.0.0 → 2.0.0 | Breaking changes (no API compatibility) |
| MINOR | 0.5.0 → 0.6.0 | New features (backwards compatible) |
| PATCH | 0.5.0 → 0.5.1 | Bug fixes and performance improvements only |

### 0.x.x Era (Current)

0.x.x means "in development" - API stability is not guaranteed before 1.0.0.

| Version | Allowed Changes |
|---------|-----------------|
| 0.MINOR.0 | New features + minor breaking changes |
| 0.MINOR.PATCH | Bug fixes, documentation, performance improvements only |

### 1.0.0 Release Criteria

- Core features are complete
- API is stable
- Documentation is comprehensive

## Release Flow

1. Develop on feature branch
2. Create PR → merge to main
3. Update CHANGELOG.md
4. Run `/kodo-release` skill
5. GitHub Release auto-generated

## GitHub Issues Workflow

### Labels

| Label | Purpose |
|-------|---------|
| `enhancement` | New feature |
| `bug` | Bug fix |
| `documentation` | Documentation |
| `type:perf` | Performance improvement |
| `priority:high` | High priority |
| `priority:medium` | Medium priority |
| `priority:low` | Low priority |
| `good first issue` | Good for newcomers |

### Milestones

| Milestone | Purpose |
|-----------|---------|
| `0.x.0` | Features for next minor release |
| `0.x.1` | Urgent hotfix (created as needed) |
| `Backlog` | Low priority / timing undecided |

### Creating Issues

```bash
# From CLI
gh issue create --title "Title" --body "Description" --label "enhancement,priority:medium" --milestone "0.6.0"

# Or use GitHub Web UI
```

### Linking PRs and Issues

Include `Fixes #123` in PR description to auto-close the issue on merge.
