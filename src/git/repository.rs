//! Git repository wrapper

use crate::config::expand_tilde;
use crate::error::{Error, Result};
use crate::git::{CommitInfo, DiffStats, FileChange};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use git2::{DiffOptions, Repository as Git2Repository};
use std::path::Path;

/// Wrapper around git2::Repository with convenience methods
pub struct Repository {
    inner: Git2Repository,
    name: String,
}

impl Repository {
    /// Open a git repository at the given path
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The path is not a git repository
    pub fn open(path: &Path, name: &str) -> Result<Self> {
        let expanded = expand_tilde(path);

        if !expanded.exists() {
            return Err(Error::RepoNotFound { path: expanded });
        }

        if !expanded.join(".git").exists() && !expanded.join("HEAD").exists() {
            return Err(Error::NotGitRepo { path: expanded });
        }

        let inner = Git2Repository::open(&expanded)?;

        Ok(Self {
            inner,
            name: name.to_string(),
        })
    }

    /// Get the repository name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get commits in the specified date range
    ///
    /// # Arguments
    ///
    /// * `from` - Start date (inclusive)
    /// * `to` - End date (inclusive)
    /// * `branch` - Optional branch name (defaults to HEAD)
    /// * `exclude_merges` - Whether to exclude merge commits
    ///
    /// # Errors
    ///
    /// Returns an error if git operations fail
    pub fn commits_in_range(
        &self,
        from: NaiveDate,
        to: NaiveDate,
        branch: Option<&str>,
        exclude_merges: bool,
    ) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.inner.revwalk()?;

        // Start from the specified branch or HEAD
        if let Some(branch_name) = branch {
            let reference = self.inner.find_reference(&format!("refs/heads/{branch_name}"))?;
            revwalk.push_ref(reference.name().unwrap_or("HEAD"))?;
        } else {
            revwalk.push_head()?;
        }

        // Sort by time (newest first)
        revwalk.set_sorting(git2::Sort::TIME)?;

        let from_datetime = Self::date_to_datetime(from);
        let to_end = Self::date_to_datetime(to);
        let to_datetime = to_end
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap_or(to_end);

        let mut commits = Vec::new();

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.inner.find_commit(oid)?;

            // Convert git timestamp to DateTime<Utc>
            let timestamp = Self::git_time_to_datetime(commit.time());

            // Skip commits outside date range
            if timestamp < from_datetime {
                break; // Since we're sorted by time, no need to continue
            }
            if timestamp >= to_datetime {
                continue;
            }

            // Check if merge commit
            let is_merge = commit.parent_count() > 1;
            if exclude_merges && is_merge {
                continue;
            }

            // Calculate diff stats
            let diff_stats = self.calculate_diff_stats(&commit)?;

            let commit_info = CommitInfo::new(
                oid.to_string()[..7].to_string(),
                timestamp,
                is_merge,
                diff_stats,
            );

            commits.push(commit_info);
        }

        Ok(commits)
    }

    /// Calculate diff statistics for a commit
    fn calculate_diff_stats(&self, commit: &git2::Commit) -> Result<DiffStats> {
        let tree = commit.tree()?;

        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = DiffOptions::new();
        diff_opts.ignore_whitespace(false);

        let diff = self.inner.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        let mut stats = DiffStats::default();

        diff.foreach(
            &mut |_, _| true,
            None,
            None,
            Some(&mut |delta, _hunk, line| {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                match line.origin() {
                    '+' => {
                        // Find or create file entry
                        if let Some(file) = stats.files.iter_mut().find(|f| f.path == path) {
                            file.additions += 1;
                        } else {
                            stats.files.push(FileChange::new(path, 1, 0));
                        }
                    }
                    '-' => {
                        if let Some(file) = stats.files.iter_mut().find(|f| f.path == path) {
                            file.deletions += 1;
                        } else {
                            stats.files.push(FileChange::new(path, 0, 1));
                        }
                    }
                    _ => {}
                }
                true
            }),
        )?;

        // Aggregate stats from files
        stats.additions = stats.files.iter().map(|f| f.additions).sum();
        stats.deletions = stats.files.iter().map(|f| f.deletions).sum();
        stats.files_changed = stats.files.len() as u32;

        Ok(stats)
    }

    /// Convert NaiveDate to DateTime<Utc> at midnight
    fn date_to_datetime(date: NaiveDate) -> DateTime<Utc> {
        Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
    }

    /// Convert git2::Time to DateTime<Utc>
    fn git_time_to_datetime(time: git2::Time) -> DateTime<Utc> {
        DateTime::from_timestamp(time.seconds(), 0).unwrap_or_else(Utc::now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::TempDir;

    fn create_test_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .unwrap();

        // Configure git user
        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(path)
            .output()
            .unwrap();

        // Create initial commit
        std::fs::write(path.join("README.md"), "# Test\n").unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(path)
            .output()
            .unwrap();

        let repo = Repository::open(path, "test-repo").unwrap();
        (dir, repo)
    }

    #[test]
    fn test_open_valid_repo() {
        let (_dir, repo) = create_test_repo();
        assert_eq!(repo.name(), "test-repo");
    }

    #[test]
    fn test_open_nonexistent_path() {
        let result = Repository::open(Path::new("/nonexistent/path"), "test");
        assert!(matches!(result, Err(Error::RepoNotFound { .. })));
    }

    #[test]
    fn test_open_not_git_repo() {
        let dir = TempDir::new().unwrap();
        let result = Repository::open(dir.path(), "test");
        assert!(matches!(result, Err(Error::NotGitRepo { .. })));
    }

    #[test]
    fn test_commits_in_range() {
        let (_dir, repo) = create_test_repo();

        let today = Utc::now().date_naive();
        let from = today - chrono::Duration::days(7);

        let commits = repo.commits_in_range(from, today, None, false).unwrap();

        // Should have at least the initial commit
        assert!(!commits.is_empty());
    }

    #[test]
    fn test_date_to_datetime() {
        use chrono::Timelike;

        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let dt = Repository::date_to_datetime(date);

        assert_eq!(dt.date_naive(), date);
        assert_eq!(dt.time().hour(), 0);
    }
}
