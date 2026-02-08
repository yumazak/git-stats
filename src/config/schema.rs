//! Configuration schema definitions

use serde::Deserialize;
use std::path::PathBuf;

/// Root configuration structure
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// JSON Schema reference (for IDE support)
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// List of repositories to analyze
    pub repositories: Vec<RepoConfig>,

    /// Default settings
    #[serde(default)]
    pub defaults: Defaults,
}

/// Single repository configuration
#[derive(Debug, Clone, Deserialize)]
pub struct RepoConfig {
    /// Display name
    pub name: String,

    /// Path to repository (supports ~)
    pub path: PathBuf,

    /// Default branch to analyze
    pub branch: Option<String>,
}

/// Default settings
#[derive(Debug, Clone, Deserialize)]
pub struct Defaults {
    /// Number of days to analyze
    #[serde(default = "default_days")]
    pub days: u32,

    /// Exclude merge commits
    #[serde(default = "default_true")]
    pub exclude_merges: bool,
}

const fn default_days() -> u32 {
    7
}

const fn default_true() -> bool {
    true
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            days: default_days(),
            exclude_merges: default_true(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let defaults = Defaults::default();
        assert_eq!(defaults.days, 7);
        assert!(defaults.exclude_merges);
    }

    #[test]
    fn test_config_deserialize() {
        let json = r#"{
            "$schema": "https://example.com/schema.json",
            "repositories": [
                {"name": "test-repo", "path": "/tmp/repo"}
            ]
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.schema, Some("https://example.com/schema.json".to_string()));
        assert_eq!(config.repositories.len(), 1);
        assert_eq!(config.repositories[0].name, "test-repo");
        assert_eq!(config.defaults.days, 7);
    }

    #[test]
    fn test_config_with_defaults() {
        let json = r#"{
            "repositories": [
                {"name": "repo", "path": "/path"}
            ],
            "defaults": {
                "days": 30,
                "exclude_merges": false
            }
        }"#;

        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.defaults.days, 30);
        assert!(!config.defaults.exclude_merges);
    }

    #[test]
    fn test_repo_config_with_branch() {
        let json = r#"{"name": "repo", "path": "/path", "branch": "main"}"#;
        let repo: RepoConfig = serde_json::from_str(json).unwrap();
        assert_eq!(repo.branch, Some("main".to_string()));
    }
}
