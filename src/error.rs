//! Error types for git-stats

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for git-stats library
#[derive(Error, Debug)]
pub enum Error {
    /// Configuration file not found at the specified path
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    /// Configuration file is invalid or malformed
    #[error("Invalid configuration: {message}")]
    ConfigInvalid { message: String },

    /// Repository directory does not exist
    #[error("Repository not found: {path}")]
    RepoNotFound { path: PathBuf },

    /// Path exists but is not a git repository
    #[error("Not a git repository: {path}")]
    NotGitRepo { path: PathBuf },

    /// Error from git2 library
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// IO error (file operations)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing/serialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// No repositories configured or specified
    #[error("No repositories to analyze")]
    NoRepositories,
}

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ConfigNotFound {
            path: PathBuf::from("/path/to/config.json"),
        };
        assert!(err.to_string().contains("/path/to/config.json"));
    }

    #[test]
    fn test_error_no_repositories() {
        let err = Error::NoRepositories;
        assert_eq!(err.to_string(), "No repositories to analyze");
    }

    #[test]
    fn test_error_not_git_repo() {
        let err = Error::NotGitRepo {
            path: PathBuf::from("/tmp/not-a-repo"),
        };
        assert!(err.to_string().contains("Not a git repository"));
    }
}
