//! # git-stats
//!
//! A CLI tool for analyzing Git commit statistics across repositories.
//!
//! ## Features
//!
//! - Analyze commit history with date range filtering
//! - Display statistics via TUI (bar chart, line chart)
//! - Export data in JSON/CSV format
//! - Filter by branch and file types

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod cli;
pub mod config;
pub mod error;
pub mod git;
pub mod output;
pub mod stats;
pub mod tui;

pub use error::{Error, Result};
