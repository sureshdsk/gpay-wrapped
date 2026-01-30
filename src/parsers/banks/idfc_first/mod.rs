//! IDFC First Bank implementation
//!
//! IDFC First Bank is a private sector bank in India formed from the merger
//! of IDFC Bank and Capital First. Their statements come in Excel (XLSX) format.

use crate::parsers::banks::base::{Bank, BankInfo, DetectionPattern, FileFormat, FormatParser};
use std::sync::Arc;

mod excel_parser;

pub use excel_parser::IdfcFirstExcelParser;

/// IDFC First Bank implementation
pub struct IDFCFirstBank {
    info: BankInfo,
    excel_parser: Arc<IdfcFirstExcelParser>,
}

impl IDFCFirstBank {
    pub fn new() -> Self {
        Self {
            info: BankInfo {
                name: "IDFC First Bank",
                code: "idfc_first",
                aliases: &[
                    "IDFC FIRST",
                    "IDFC First",
                    "IDFC First Bank",
                    "IDFCFirstBank",
                    "IDFCFIRST",
                ],
                detection_patterns: &[
                    DetectionPattern::ContentContains(&[
                        "IDFC FIRST",
                        "IDFC First Bank",
                        "IDFCFirstBank",
                    ]),
                    DetectionPattern::FilenamePattern(r"(?i)idfc.*first.*statement"),
                    DetectionPattern::FilenamePattern(r"(?i)idfcfirst.*bank.*statement"),
                    DetectionPattern::FilenamePattern(r"(?i)IDFCFIRSTBank"),
                ],
            },
            excel_parser: Arc::new(IdfcFirstExcelParser::new()),
        }
    }
}

impl Default for IDFCFirstBank {
    fn default() -> Self {
        Self::new()
    }
}

impl Bank for IDFCFirstBank {
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
    fn test_idfc_first_bank_info() {
        let bank = IDFCFirstBank::new();
        assert_eq!(bank.info().name, "IDFC First Bank");
        assert_eq!(bank.info().code, "idfc_first");
        assert!(!bank.info().aliases.is_empty());
    }

    #[test]
    fn test_idfc_first_can_handle_filename() {
        let bank = IDFCFirstBank::new();
        assert!(bank.can_handle("IDFCFIRSTBankstatement_12345.xlsx", None));
        assert!(bank.can_handle("idfc_first_statement.xlsx", None));
        assert!(!bank.can_handle("HDFC_Statement.xlsx", None));
    }

    #[test]
    fn test_idfc_first_get_parser() {
        let bank = IDFCFirstBank::new();
        assert!(bank.get_parser(FileFormat::Excel).is_some());
        assert!(bank.get_parser(FileFormat::Ofx).is_none());
    }
}
