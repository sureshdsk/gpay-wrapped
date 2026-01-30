//! Core traits and types for bank-specific statement parsers
//!
//! This module defines the foundational abstractions for implementing bank-specific
//! parsers while maintaining a consistent interface across all banks.

use crate::parsers::base::{ParseResult, ParserOptions, ParserResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;

/// File format enumeration for statement files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileFormat {
    Excel,
    Ofx,
    Qfx,
}

impl FileFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &str {
        match self {
            FileFormat::Excel => "xlsx",
            FileFormat::Ofx => "ofx",
            FileFormat::Qfx => "qfx",
        }
    }

    /// Get the format as a string identifier
    pub fn as_str(&self) -> &str {
        match self {
            FileFormat::Excel => "excel",
            FileFormat::Ofx => "ofx",
            FileFormat::Qfx => "qfx",
        }
    }

    /// Parse from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "xlsx" | "xls" => Some(FileFormat::Excel),
            "ofx" => Some(FileFormat::Ofx),
            "qfx" => Some(FileFormat::Qfx),
            _ => None,
        }
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Detection pattern strategies for identifying banks from statements
#[derive(Debug, Clone)]
pub enum DetectionPattern {
    /// Content contains specific keyword(s)
    ContentContains(&'static [&'static str]),

    /// Content matches a regular expression
    ContentRegex(&'static str),

    /// Filename matches a pattern
    FilenamePattern(&'static str),

    /// Account number format (regex pattern)
    AccountNumberRegex(&'static str),
}

impl DetectionPattern {
    /// Test if content matches this detection pattern
    pub fn matches_content(&self, content: &str) -> bool {
        match self {
            DetectionPattern::ContentContains(keywords) => {
                let content_lower = content.to_lowercase();
                keywords
                    .iter()
                    .any(|&kw| content_lower.contains(&kw.to_lowercase()))
            }
            DetectionPattern::ContentRegex(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(content)
                } else {
                    false
                }
            }
            _ => false, // Other patterns checked separately
        }
    }

    /// Test if filename matches this detection pattern
    pub fn matches_filename(&self, filename: &str) -> bool {
        match self {
            DetectionPattern::FilenamePattern(pattern) => {
                if let Ok(re) = Regex::new(pattern) {
                    re.is_match(filename)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

/// Static information about a bank
#[derive(Debug, Clone)]
pub struct BankInfo {
    /// Full bank name
    pub name: &'static str,

    /// Short bank code (e.g., "icici", "hdfc")
    pub code: &'static str,

    /// Alternative names/aliases for detection
    pub aliases: &'static [&'static str],

    /// Detection patterns for automatic bank identification
    pub detection_patterns: &'static [DetectionPattern],
}

impl BankInfo {
    /// Check if content matches any detection pattern
    pub fn matches_content(&self, content: &str) -> bool {
        self.detection_patterns
            .iter()
            .any(|pattern| pattern.matches_content(content))
    }

    /// Check if filename matches any detection pattern
    pub fn matches_filename(&self, filename: &str) -> bool {
        // Check aliases first
        let filename_lower = filename.to_lowercase();
        if self.aliases.iter().any(|&alias| {
            filename_lower.contains(&alias.to_lowercase())
                || filename_lower.eq_ignore_ascii_case(alias)
        }) {
            return true;
        }

        // Check patterns
        self.detection_patterns
            .iter()
            .any(|pattern| pattern.matches_filename(filename))
    }
}

/// Result of bank detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    /// Detected bank code
    pub bank: String,

    /// Detection confidence score (0.0 to 1.0)
    pub confidence: f32,

    /// Detected file format
    pub format: FileFormat,

    /// Suggested parser name
    pub suggested_parser: String,

    /// Reason for detection (for logging/debugging)
    pub detection_reason: String,
}

/// Base trait for format-specific parsers
///
/// Each format parser (PDF, CSV, etc.) implements this trait to provide
/// format-specific parsing logic for a particular bank.
pub trait FormatParser: Send + Sync {
    /// Get the format this parser handles
    fn format(&self) -> FileFormat;

    /// Get the bank code this parser is for
    fn bank_code(&self) -> &str;

    /// Get the parser name (auto-generated from bank code and format)
    fn name(&self) -> String {
        format!("{}-{}", self.bank_code(), self.format().as_str())
    }

    /// Check if this parser can handle the given file
    fn can_parse(&self, file_path: &str, _content: Option<&[u8]>) -> bool {
        // Default: check file extension
        if let Some(ext) = file_path.rsplit('.').next() {
            if let Some(format) = FileFormat::from_extension(ext) {
                return format == self.format();
            }
        }
        false
    }

    /// Parse a statement file and return transactions
    fn parse(
        &self,
        file_path: &str,
        options: &ParserOptions,
    ) -> ParserResult<ParseResult>;

    /// Parse from bytes (for uploaded files)
    fn parse_bytes(&self, data: &[u8], options: &ParserOptions) -> ParserResult<ParseResult>;
}

/// Base trait for bank implementations
///
/// Each bank (ICICI, HDFC, SBI, etc.) implements this trait to provide
/// bank-specific parsers and detection logic.
pub trait Bank: Send + Sync {
    /// Get static information about this bank
    fn info(&self) -> &BankInfo;

    /// Check if this bank can handle the given file
    fn can_handle(&self, file_path: &str, content: Option<&[u8]>) -> bool;

    /// Get all available parsers for this bank (PDF, CSV, etc.)
    fn parsers(&self) -> Vec<&dyn FormatParser>;

    /// Get parser for a specific format
    fn get_parser(&self, format: FileFormat) -> Option<&dyn FormatParser>;

    /// Detect if content matches this bank and return confidence score
    fn detect_confidence(&self, file_path: &str, content: &str) -> f32 {
        let mut confidence = 0.0f32;

        // Check filename match
        if self.info().matches_filename(file_path) {
            confidence += 0.4;
        }

        // Check content match
        if self.info().matches_content(content) {
            confidence += 0.6;
        }

        confidence.min(1.0)
    }
}

/// Helper to downcast a trait object to a concrete type
///
/// This is useful when you need to access bank-specific methods
/// after retrieving a bank from the registry.
pub trait AsAny {
    /// Get as `Any` for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: std::any::Any> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format_extension() {
        assert_eq!(FileFormat::Excel.extension(), "xlsx");
        assert_eq!(FileFormat::Ofx.extension(), "ofx");
        assert_eq!(FileFormat::Qfx.extension(), "qfx");
    }

    #[test]
    fn test_file_format_from_extension() {
        assert_eq!(FileFormat::from_extension("xlsx"), Some(FileFormat::Excel));
        assert_eq!(FileFormat::from_extension("xls"), Some(FileFormat::Excel));
        assert_eq!(FileFormat::from_extension("XLS"), Some(FileFormat::Excel));
        assert_eq!(FileFormat::from_extension("ofx"), Some(FileFormat::Ofx));
        assert_eq!(FileFormat::from_extension("txt"), None);
    }

    #[test]
    fn test_detection_pattern_content_contains() {
        let pattern = DetectionPattern::ContentContains(&["ICICI", "Industrial Credit"]);
        assert!(pattern.matches_content("ICICI Bank Statement"));
        assert!(pattern.matches_content("Industrial Credit and Investment Corporation"));
        assert!(!pattern.matches_content("HDFC Bank Statement"));
    }

    #[test]
    fn test_detection_pattern_filename() {
        let pattern = DetectionPattern::FilenamePattern(r"(?i)icici.*statement");
        assert!(pattern.matches_filename("ICICI_Statement_Jan2025.xlsx"));
        assert!(pattern.matches_filename("icici_statement_dec.xls"));
        assert!(!pattern.matches_filename("HDFC_Statement.xlsx"));
    }

    #[test]
    fn test_bank_info_matches() {
        let info = BankInfo {
            name: "ICICI Bank",
            code: "icici",
            aliases: &["ICICI", "Industrial Credit"],
            detection_patterns: &[
                DetectionPattern::ContentContains(&["ICICI Bank"]),
                DetectionPattern::FilenamePattern(r"(?i)icici.*statement"),
            ],
        };

        assert!(info.matches_filename("ICICI_Statement.xlsx"));
        assert!(info.matches_content("ICICI Bank Statement for Account"));
        assert!(!info.matches_filename("HDFC_Statement.xlsx"));
    }
}
