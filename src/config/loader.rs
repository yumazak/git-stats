//! Configuration loading and path utilities

use crate::config::Config;
use crate::error::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Load configuration from a JSON file
///
/// # Errors
///
/// Returns an error if:
/// - The file does not exist
/// - The file cannot be read
/// - The JSON is invalid
pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Err(Error::ConfigNotFound {
            path: path.to_path_buf(),
        });
    }

    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;

    // Validate that we have at least one repository
    if config.repositories.is_empty() {
        return Err(Error::ConfigInvalid {
            message: "No repositories configured".to_string(),
        });
    }

    Ok(config)
}

/// Get the default configuration file path
///
/// Checks in order:
/// 1. `~/.config/git-stats/config.json` (XDG style, preferred)
/// 2. Platform-specific config dir (e.g., `~/Library/Application Support` on macOS)
#[must_use]
pub fn default_config_path() -> Option<PathBuf> {
    // First, try XDG-style path (~/.config/git-stats/config.json)
    if let Some(home) = dirs::home_dir() {
        let xdg_path = home.join(".config").join("git-stats").join("config.json");
        if xdg_path.exists() {
            return Some(xdg_path);
        }
    }

    // Fall back to platform-specific config dir
    dirs::config_dir().map(|p| p.join("git-stats").join("config.json"))
}

/// Expand `~` to the home directory in a path
///
/// If the path starts with `~`, it will be replaced with the home directory.
/// Otherwise, the path is returned as-is.
#[must_use]
pub fn expand_tilde(path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy();

    if path_str.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            if path_str == "~" {
                return home;
            }
            if let Some(rest) = path_str.strip_prefix("~/") {
                return home.join(rest);
            }
        }
    }

    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_not_found() {
        let result = load_config(Path::new("/nonexistent/path/config.json"));
        assert!(matches!(result, Err(Error::ConfigNotFound { .. })));
    }

    #[test]
    fn test_load_config_valid() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{"repositories": [{{"name": "test", "path": "/tmp"}}]}}"#
        )
        .unwrap();

        let config = load_config(file.path()).unwrap();
        assert_eq!(config.repositories.len(), 1);
        assert_eq!(config.repositories[0].name, "test");
    }

    #[test]
    fn test_load_config_empty_repos() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"{{"repositories": []}}"#).unwrap();

        let result = load_config(file.path());
        assert!(matches!(result, Err(Error::ConfigInvalid { .. })));
    }

    #[test]
    fn test_load_config_invalid_json() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "not valid json").unwrap();

        let result = load_config(file.path());
        assert!(matches!(result, Err(Error::Json(_))));
    }

    #[test]
    fn test_expand_tilde_home() {
        let expanded = expand_tilde(Path::new("~"));
        // Should not be "~" anymore if home exists
        if dirs::home_dir().is_some() {
            assert_ne!(expanded.to_string_lossy(), "~");
        }
    }

    #[test]
    fn test_expand_tilde_subpath() {
        let expanded = expand_tilde(Path::new("~/some/path"));
        if let Some(home) = dirs::home_dir() {
            assert_eq!(expanded, home.join("some/path"));
        }
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let path = Path::new("/absolute/path");
        let expanded = expand_tilde(path);
        assert_eq!(expanded, path);
    }

    #[test]
    fn test_default_config_path() {
        let path = default_config_path();
        // Should return Some on most systems
        if dirs::config_dir().is_some() {
            assert!(path.is_some());
            let p = path.unwrap();
            assert!(p.to_string_lossy().contains("git-stats"));
            assert!(p.to_string_lossy().contains("config.json"));
        }
    }
}
