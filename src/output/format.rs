//! Output formatter trait

use crate::error::Result;
use crate::stats::AnalysisResult;

/// Trait for output formatters
pub trait Formatter {
    /// Format the analysis result as a string
    ///
    /// # Errors
    ///
    /// Returns an error if formatting fails
    fn format(&self, result: &AnalysisResult) -> Result<String>;
}
