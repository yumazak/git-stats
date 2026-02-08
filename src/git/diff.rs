//! Diff statistics types

/// Diff statistics for a commit
#[derive(Debug, Clone, Default)]
pub struct DiffStats {
    /// Total lines added
    pub additions: u64,

    /// Total lines deleted
    pub deletions: u64,

    /// Number of files changed
    pub files_changed: u32,

    /// Per-file changes
    pub files: Vec<FileChange>,
}

impl DiffStats {
    /// Create a new `DiffStats` with values
    #[must_use]
    pub fn new(additions: u64, deletions: u64, files_changed: u32) -> Self {
        Self {
            additions,
            deletions,
            files_changed,
            files: Vec::new(),
        }
    }

    /// Calculate net line change (additions - deletions)
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub fn net_lines(&self) -> i64 {
        self.additions as i64 - self.deletions as i64
    }

    /// Add a file change to the statistics
    pub fn add_file(&mut self, file: FileChange) {
        self.additions += file.additions;
        self.deletions += file.deletions;
        self.files_changed += 1;
        self.files.push(file);
    }
}

/// Individual file change within a commit
#[derive(Debug, Clone)]
pub struct FileChange {
    /// File path
    pub path: String,

    /// Lines added in this file
    pub additions: u64,

    /// Lines deleted in this file
    pub deletions: u64,
}

impl FileChange {
    /// Create a new `FileChange`
    #[must_use]
    pub fn new(path: String, additions: u64, deletions: u64) -> Self {
        Self {
            path,
            additions,
            deletions,
        }
    }

    /// Check if file matches any of the given extensions
    #[must_use]
    pub fn matches_extensions(&self, extensions: &[String]) -> bool {
        if extensions.is_empty() {
            return true;
        }

        let path = std::path::Path::new(&self.path);
        path.extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| extensions.iter().any(|e| e == ext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_stats_net_lines() {
        let stats = DiffStats::new(100, 30, 5);
        assert_eq!(stats.net_lines(), 70);
    }

    #[test]
    fn test_diff_stats_negative_net() {
        let stats = DiffStats::new(10, 50, 3);
        assert_eq!(stats.net_lines(), -40);
    }

    #[test]
    fn test_diff_stats_add_file() {
        let mut stats = DiffStats::default();
        stats.add_file(FileChange::new("src/main.rs".to_string(), 10, 5));
        stats.add_file(FileChange::new("src/lib.rs".to_string(), 20, 3));

        assert_eq!(stats.additions, 30);
        assert_eq!(stats.deletions, 8);
        assert_eq!(stats.files_changed, 2);
        assert_eq!(stats.files.len(), 2);
    }

    #[test]
    fn test_file_change_matches_extensions() {
        let file = FileChange::new("src/main.rs".to_string(), 10, 5);

        assert!(file.matches_extensions(&["rs".to_string(), "ts".to_string()]));
        assert!(!file.matches_extensions(&["ts".to_string(), "js".to_string()]));
        assert!(file.matches_extensions(&[])); // Empty = match all
    }

    #[test]
    fn test_file_change_no_extension() {
        let file = FileChange::new("Makefile".to_string(), 10, 5);

        assert!(!file.matches_extensions(&["rs".to_string()]));
        assert!(file.matches_extensions(&[]));
    }
}
