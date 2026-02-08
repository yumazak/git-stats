//! Configuration module for kodo

pub mod loader;
pub mod schema;

pub use loader::{
    default_config_path, default_config_path_for_save, expand_tilde, load_config, save_config,
};
pub use schema::{Config, Defaults, RepoConfig};
