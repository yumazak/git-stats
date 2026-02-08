//! CLI module for kodo

pub mod args;
pub mod run;

pub use args::{AddArgs, Args, Command};
pub use run::execute;
