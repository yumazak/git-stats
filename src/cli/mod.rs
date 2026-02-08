//! CLI module for git-stats

pub mod args;
pub mod run;

pub use args::Args;
pub use run::execute;
