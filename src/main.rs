//! git-stats CLI entry point

use clap::Parser;
use git_stats::cli::{Args, execute};
use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();

    if let Err(e) = execute(args) {
        eprintln!("error: {e}");

        // Print cause chain
        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {cause}");
            source = cause.source();
        }

        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
