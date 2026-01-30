//! Bank detection module
//!
//! This module provides automatic detection of banks from statement files
//! using multiple strategies: filename patterns, content analysis, and format detection.

use crate::parsers::banks::base::{Bank, DetectionResult, FileFormat};
use crate::parsers::base::ParserError;
use std::path::Path;
use std::sync::Arc;

/// Bank detector for automatic bank identification
pub struct BankDetector {
    banks: Vec<Arc<dyn Bank>>,
}

impl BankDetector {
    /// Create a new bank detector
    pub fn new() -> Self {
        Self {
            banks: Vec::new(),
        }
    }

    /// Register a bank for detection
    pub fn register_bank(&mut self, bank: Arc<dyn Bank>) {
        self.banks.push(bank);
    }

    /// Detect bank from file
    ///
    /// This is the main detection method that combines multiple strategies:
    /// 1. Filename pattern matching
    /// 2. Content keyword analysis
    /// 3. Format detection from file extension
    pub fn detect(&self, file_path: &str, content: &[u8]) -> Result<DetectionResult, ParserError> {
        // Convert bytes to string for content analysis
        let content_str = String::from_utf8_lossy(content);

        // Detect format from file extension
        let format = self.detect_format(file_path)?;

        // Detect bank from filename and content
        let detection = self.detect_from_content(&content_str, file_path, format);

        detection.ok_or_else(|| {
            ParserError::ParseError(format!(
                "Could not detect bank from file: {}",
                file_path
            ))
        })
    }

    /// Detect bank from text content
    pub fn detect_from_content(
        &self,
        content: &str,
        file_path: &str,
        format: FileFormat,
    ) -> Option<DetectionResult> {
        let mut best_match: Option<DetectionResult> = None;
        let mut best_confidence = 0.0f32;

        // Check each registered bank
        for bank in &self.banks {
            let confidence = bank.detect_confidence(file_path, content);

            if confidence > best_confidence {
                best_confidence = confidence;

                // Get the appropriate parser for this format
                let parser = bank.get_parser(format);

                if let Some(parser) = parser {
                    let detection_reason = if confidence > 0.8 {
                        "Strong match from filename and content".to_string()
                    } else if confidence > 0.5 {
                        "Moderate match from filename or content".to_string()
                    } else {
                        "Weak match, low confidence".to_string()
                    };

                    best_match = Some(DetectionResult {
                        bank: bank.info().code.to_string(),
                        confidence,
                        format,
                        suggested_parser: parser.name(),
                        detection_reason,
                    });
                }
            }
        }

        // Only return if confidence is above threshold
        if best_confidence >= 0.3 {
            best_match
        } else {
            None
        }
    }

    /// Detect bank from filename only
    pub fn detect_from_filename(&self, filename: &str) -> Option<DetectionResult> {
        let path = Path::new(filename);
        let file_name = path.file_name()?.to_str()?;

        let format = self.detect_format(filename).ok()?;

        for bank in &self.banks {
            if bank.info().matches_filename(file_name) {
                if let Some(parser) = bank.get_parser(format) {
                    return Some(DetectionResult {
                        bank: bank.info().code.to_string(),
                        confidence: 0.7, // Filename-only match gets moderate confidence
                        format,
                        suggested_parser: parser.name(),
                        detection_reason: "Matched from filename pattern".to_string(),
                    });
                }
            }
        }

        None
    }

    /// Detect file format from extension
    pub fn detect_format(&self, file_path: &str) -> Result<FileFormat, ParserError> {
        let path = Path::new(file_path);
        let extension = path.extension().and_then(|ext| ext.to_str()).ok_or_else(|| {
            ParserError::ParseError(format!("No file extension found: {}", file_path))
        })?;

        FileFormat::from_extension(extension).ok_or_else(|| {
            ParserError::UnsupportedFormat(format!("Unsupported file extension: {}", extension))
        })
    }

    /// Get list of registered bank codes
    pub fn registered_banks(&self) -> Vec<&str> {
        self.banks.iter().map(|b| b.info().code).collect()
    }

    /// Get bank info by code
    pub fn get_bank_info(&self, code: &str) -> Option<&dyn Bank> {
        self.banks
            .iter()
            .find(|b| b.info().code == code)
            .map(|b| b.as_ref())
    }
}

impl Default for BankDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::banks::base::{
        BankInfo, DetectionPattern, FileFormat, FormatParser,
    };
    use crate::parsers::base::{ParseResult, ParserOptions, ParserResult};
    use std::sync::Arc;

    // Mock format parser for testing
    struct MockParser;

    impl FormatParser for MockParser {
        fn format(&self) -> FileFormat {
            FileFormat::Excel
        }

        fn bank_code(&self) -> &str {
            "test"
        }

        fn parse(&self, _file_path: &str, _options: &ParserOptions) -> ParserResult<ParseResult> {
            Ok(ParseResult::new(vec![]))
        }

        fn parse_bytes(&self, _data: &[u8], _options: &ParserOptions) -> ParserResult<ParseResult> {
            Ok(ParseResult::new(vec![]))
        }
    }

    // Mock bank for testing
    struct MockBank {
        info: BankInfo,
        parser: MockParser,
    }

    impl MockBank {
        fn new() -> Self {
            Self {
                info: BankInfo {
                    name: "Test Bank",
                    code: "test",
                    aliases: &["TEST", "Test Bank"],
                    detection_patterns: &[
                        DetectionPattern::ContentContains(&["Test Bank", "TEST STATEMENT"]),
                        DetectionPattern::FilenamePattern(r"(?i)test.*statement"),
                    ],
                },
                parser: MockParser,
            }
        }
    }

    impl Bank for MockBank {
        fn info(&self) -> &BankInfo {
            &self.info
        }

        fn can_handle(&self, file_path: &str, _content: Option<&[u8]>) -> bool {
            self.info.matches_filename(file_path)
        }

        fn parsers(&self) -> Vec<&dyn FormatParser> {
            vec![&self.parser]
        }

        fn get_parser(&self, format: FileFormat) -> Option<&dyn FormatParser> {
            if format == FileFormat::Excel {
                Some(&self.parser)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_detect_format() {
        let detector = BankDetector::new();

        assert_eq!(
            detector.detect_format("statement.xlsx").unwrap(),
            FileFormat::Excel
        );
        assert_eq!(
            detector.detect_format("statement.xls").unwrap(),
            FileFormat::Excel
        );
        assert_eq!(
            detector.detect_format("statement.ofx").unwrap(),
            FileFormat::Ofx
        );
        assert!(detector.detect_format("statement.txt").is_err());
    }

    #[test]
    fn test_register_and_list_banks() {
        let mut detector = BankDetector::new();
        assert_eq!(detector.registered_banks().len(), 0);

        detector.register_bank(Arc::new(MockBank::new()));
        assert_eq!(detector.registered_banks().len(), 1);
        assert_eq!(detector.registered_banks()[0], "test");
    }

    #[test]
    fn test_detect_from_filename() {
        let mut detector = BankDetector::new();
        detector.register_bank(Arc::new(MockBank::new()));

        let result = detector.detect_from_filename("TEST_Statement.xlsx");
        assert!(result.is_some());
        assert_eq!(result.unwrap().bank, "test");
    }

    #[test]
    fn test_detect_from_content() {
        let mut detector = BankDetector::new();
        detector.register_bank(Arc::new(MockBank::new()));

        let content = "This is a Test Bank statement";
        let result = detector.detect_from_content(content, "statement.xlsx", FileFormat::Excel);

        assert!(result.is_some());
        assert_eq!(result.unwrap().bank, "test");
    }

    #[test]
    fn test_detect_confidence_threshold() {
        let mut detector = BankDetector::new();
        detector.register_bank(Arc::new(MockBank::new()));

        // Low confidence content should not match
        let content = "Generic statement with no bank identifiers";
        let result = detector.detect_from_content(content, "generic.xlsx", FileFormat::Excel);

        assert!(result.is_none());
    }
}
