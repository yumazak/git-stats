//! Configuration module for kodo

pub mod loader;
pub mod schema;

pub use loader::{default_config_path, expand_tilde, load_config};
pub use schema::{Config, Defaults, RepoConfig};
