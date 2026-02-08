//! Git repository interface module

pub mod commit;
pub mod diff;
pub mod repository;

pub use commit::CommitInfo;
pub use diff::{DiffStats, FileChange};
pub use repository::Repository;
