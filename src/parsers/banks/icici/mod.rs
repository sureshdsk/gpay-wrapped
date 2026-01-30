//! ICICI Bank implementation
//!
//! ICICI Bank is one of the major private sector banks in India.
//! Their statements come in Excel (XLS) format.

use crate::parsers::banks::base::{Bank, BankInfo, DetectionPattern, FileFormat, FormatParser};
use std::sync::Arc;

mod excel_parser;

pub use excel_parser::IciciExcelParser;

/// ICICI Bank implementation
pub struct ICICIBank {
    info: BankInfo,
    excel_parser: Arc<IciciExcelParser>,
}

impl ICICIBank {
    pub fn new() -> Self {
        Self {
            info: BankInfo {
                name: "ICICI Bank",
                code: "icici",
                aliases: &[
                    "ICICI",
                    "ICICI Bank",
                    "Industrial Credit and Investment Corporation of India",
                ],
                detection_patterns: &[
                    DetectionPattern::ContentContains(&[
                        "ICICI Bank",
                        "Industrial Credit and Investment Corporation",
                        "ICICI Ltd",
                    ]),
                    DetectionPattern::FilenamePattern(r"(?i)icici.*statement"),
                ],
            },
            excel_parser: Arc::new(IciciExcelParser::new()),
        }
    }
}

impl Default for ICICIBank {
    fn default() -> Self {
        Self::new()
    }
}

impl Bank for ICICIBank {
    fn info(&self) -> &BankInfo {
        &self.info
    }

    fn can_handle(&self, file_path: &str, content: Option<&[u8]>) -> bool {
        // Check filename
        if self.info.matches_filename(file_path) {
            return true;
        }

        // Check content if provided
        if let Some(data) = content {
            if let Ok(content_str) = String::from_utf8(data.to_vec()) {
                if self.info.matches_content(&content_str) {
                    return true;
                }
            }
        }

        false
    }

    fn parsers(&self) -> Vec<&dyn FormatParser> {
        vec![self.excel_parser.as_ref() as &dyn FormatParser]
    }

    fn get_parser(&self, format: FileFormat) -> Option<&dyn FormatParser> {
        match format {
            FileFormat::Excel => Some(self.excel_parser.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icici_bank_info() {
        let bank = ICICIBank::new();
        assert_eq!(bank.info().name, "ICICI Bank");
        assert_eq!(bank.info().code, "icici");
        assert!(!bank.info().aliases.is_empty());
    }

    #[test]
    fn test_icici_can_handle_filename() {
        let bank = ICICIBank::new();
        assert!(bank.can_handle("ICICI_Statement_Dec2024.xls", None));
        assert!(bank.can_handle("icici_bank_statement.xlsx", None));
        assert!(!bank.can_handle("HDFC_Statement.xls", None));
    }

    #[test]
    fn test_icici_get_parser() {
        let bank = ICICIBank::new();
        assert!(bank.get_parser(FileFormat::Excel).is_some());
        assert!(bank.get_parser(FileFormat::Ofx).is_none());
    }
}
