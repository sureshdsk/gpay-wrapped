//! Generic Excel parsing utilities
//!
//! This module provides base utilities for parsing Excel statements (XLS/XLSX)
//! that can be reused across different bank-specific implementations.

use calamine::{open_workbook_auto_from_rs, Data, Reader, Sheets};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::io::Cursor;
use std::str::FromStr;

/// Column mapping for Excel files
#[derive(Debug, Default, Clone)]
pub struct ExcelColumnMapping {
    /// Date column index
    pub date: Option<usize>,
    /// Posted/value date column index
    pub posted_date: Option<usize>,
    /// Description column index
    pub description: Option<usize>,
    /// Amount column index (single amount column)
    pub amount: Option<usize>,
    /// Debit/withdrawal column index
    pub debit: Option<usize>,
    /// Credit/deposit column index
    pub credit: Option<usize>,
    /// Balance column index
    pub balance: Option<usize>,
    /// Reference/check number column index
    pub reference: Option<usize>,
    /// Transaction type/mode column index
    pub transaction_type: Option<usize>,
}

impl ExcelColumnMapping {
    /// Check if this mapping has minimum required columns
    pub fn is_valid(&self) -> bool {
        self.date.is_some()
            && (self.amount.is_some() || self.debit.is_some() || self.credit.is_some())
    }
}

/// Date parsing utilities for Excel files
pub struct ExcelDateParser;

impl ExcelDateParser {
    /// Parse a date from an Excel cell (handles both string and numeric formats)
    pub fn parse_cell(cell: &Data) -> Option<NaiveDate> {
        match cell {
            Data::DateTime(dt) => {
                // calamine ExcelDateTime - convert to NaiveDate using as_datetime
                if let Some(datetime) = dt.as_datetime() {
                    Some(datetime.date())
                } else {
                    None
                }
            }
            Data::DateTimeIso(s) => {
                // ISO format date string
                NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
            }
            Data::Float(f) => {
                // Excel serial date number
                Self::from_excel_serial(*f)
            }
            Data::Int(i) => {
                // Excel serial date number as integer
                Self::from_excel_serial(*i as f64)
            }
            Data::String(s) => Self::parse_string(s),
            _ => None,
        }
    }

    /// Parse a date from a string with various formats
    pub fn parse_string(text: &str) -> Option<NaiveDate> {
        let trimmed = text.trim();

        // Common date formats used in bank statements
        let formats = [
            "%d-%m-%Y",   // 31-12-2024
            "%d/%m/%Y",   // 31/12/2024
            "%d-%m-%y",   // 31-12-24
            "%d/%m/%y",   // 31/12/24
            "%d %b %Y",   // 31 Dec 2024
            "%d-%b-%Y",   // 31-Dec-2024 or 16-Jan-2025
            "%d %B %Y",   // 31 December 2024
            "%Y-%m-%d",   // 2024-12-31
            "%Y/%m/%d",   // 2024/12/31
            "%m-%d-%Y",   // 12-31-2024 (US format)
            "%m/%d/%Y",   // 12/31/2024 (US format)
            "%b %d %Y",   // Dec 31 2024
            "%B %d %Y",   // December 31 2024
            "%d-%b-%y",   // 16-Jan-25
        ];

        for fmt in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(trimmed, fmt) {
                return Some(date);
            }
        }

        None
    }

    /// Convert Excel serial date number to NaiveDate
    pub fn from_excel_serial(serial: f64) -> Option<NaiveDate> {
        // Excel serial date: days since 1899-12-30 (with a bug for 1900-02-29)
        // We use the more common 1900 date system
        if serial < 1.0 {
            return None;
        }

        // Excel incorrectly treats 1900 as a leap year
        // Serial number 60 is 1900-02-29 (which doesn't exist)
        let adjusted = if serial >= 60.0 {
            serial - 1.0
        } else {
            serial
        };

        let base_date = NaiveDate::from_ymd_opt(1899, 12, 30)?;
        base_date.checked_add_signed(chrono::Duration::days(adjusted as i64))
    }
}

/// Amount parsing utilities for Excel files
pub struct ExcelAmountParser;

impl ExcelAmountParser {
    /// Parse an amount from an Excel cell
    pub fn parse_cell(cell: &Data) -> Option<Decimal> {
        match cell {
            Data::Float(f) => Decimal::from_f64_retain(*f),
            Data::Int(i) => Some(Decimal::from(*i)),
            Data::String(s) => Self::parse_string(s),
            _ => None,
        }
    }

    /// Parse an amount from a string, handling various formats
    pub fn parse_string(text: &str) -> Option<Decimal> {
        let cleaned = Self::clean(text)?;
        if cleaned.is_empty() {
            return None;
        }
        Decimal::from_str(&cleaned).ok()
    }

    /// Clean an amount string by removing currency symbols and formatting
    pub fn clean(text: &str) -> Option<String> {
        let mut cleaned = text.trim().to_string();

        if cleaned.is_empty() || cleaned == "-" || cleaned == "0" {
            return None;
        }

        // Remove currency symbols
        for symbol in &["$", "Rs.", "Rs", "INR", "USD", "EUR", "GBP"] {
            cleaned = cleaned.replace(symbol, "");
        }

        // Remove thousand separators
        cleaned = cleaned.replace(',', "");

        // Remove whitespace
        cleaned = cleaned.split_whitespace().collect();

        // Handle negative amounts in parentheses: (100.00) -> -100.00
        if cleaned.starts_with('(') && cleaned.ends_with(')') {
            cleaned = format!("-{}", &cleaned[1..cleaned.len() - 1]);
        }

        // Handle CR/DR suffixes (sometimes used in Indian banks)
        if cleaned.ends_with("CR") {
            cleaned = cleaned.replace("CR", "");
        } else if cleaned.ends_with("Dr") || cleaned.ends_with("DR") {
            cleaned = format!("-{}", cleaned.replace("Dr", "").replace("DR", ""));
        }

        if cleaned.is_empty() {
            return None;
        }

        Some(cleaned)
    }

    /// Determine transaction type from amount sign
    pub fn get_type_from_amount(amount: &Decimal) -> crate::parsers::base::TransactionType {
        if amount.is_sign_negative() {
            crate::parsers::base::TransactionType::Debit
        } else {
            crate::parsers::base::TransactionType::Credit
        }
    }
}

/// Column detection utilities for Excel files
pub struct ExcelColumnDetector;

impl ExcelColumnDetector {
    /// Detect column indices from header row
    pub fn detect_columns(headers: &[String]) -> ExcelColumnMapping {
        let mut mapping = ExcelColumnMapping::default();

        for (i, header) in headers.iter().enumerate() {
            let lower = header.to_lowercase();

            // Date columns
            if lower.contains("date") && mapping.date.is_none() {
                if lower.contains("post") || lower.contains("value") || lower.contains("txn") {
                    if mapping.posted_date.is_none() {
                        mapping.posted_date = Some(i);
                    }
                } else {
                    mapping.date = Some(i);
                }
            }
            // Description columns
            else if (lower.contains("description")
                || lower.contains("particulars")
                || lower.contains("narration")
                || lower.contains("details")
                || lower.contains("remark"))
                && mapping.description.is_none()
            {
                mapping.description = Some(i);
            }
            // Single amount column
            else if lower.contains("amount") && mapping.amount.is_none() {
                mapping.amount = Some(i);
            }
            // Debit columns
            else if (lower.contains("debit")
                || lower.contains("withdrawal")
                || lower.contains("withdraw")
                || lower == "dr")
                && mapping.debit.is_none()
            {
                mapping.debit = Some(i);
            }
            // Credit columns
            else if (lower.contains("credit")
                || lower.contains("deposit")
                || lower == "cr")
                && mapping.credit.is_none()
            {
                mapping.credit = Some(i);
            }
            // Balance column
            else if lower.contains("balance") && mapping.balance.is_none() {
                mapping.balance = Some(i);
            }
            // Reference columns
            else if (lower.contains("ref")
                || lower.contains("cheque")
                || lower.contains("check")
                || lower.contains("transaction id")
                || lower.contains("txn id"))
                && mapping.reference.is_none()
            {
                mapping.reference = Some(i);
            }
            // Type/Mode columns
            else if (lower.contains("type") || lower.contains("mode") || lower.contains("category"))
                && mapping.transaction_type.is_none()
            {
                mapping.transaction_type = Some(i);
            }
        }

        mapping
    }
}

/// Excel workbook reader wrapper
pub struct ExcelReader {
    workbook: Sheets<Cursor<Vec<u8>>>,
}

impl ExcelReader {
    /// Open an Excel file from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let cursor = Cursor::new(data.to_vec());
        let workbook = open_workbook_auto_from_rs(cursor)
            .map_err(|e| format!("Failed to open Excel file: {}", e))?;

        Ok(Self { workbook })
    }

    /// Get sheet names
    pub fn sheet_names(&self) -> Vec<String> {
        self.workbook.sheet_names().to_vec()
    }

    /// Get all rows from the first sheet
    pub fn get_rows(&mut self) -> Result<Vec<Vec<Data>>, String> {
        let sheets = self.workbook.sheet_names().to_vec();
        if sheets.is_empty() {
            return Err("No sheets found in workbook".to_string());
        }

        self.get_rows_from_sheet(&sheets[0])
    }

    /// Get all rows from a specific sheet
    pub fn get_rows_from_sheet(&mut self, sheet_name: &str) -> Result<Vec<Vec<Data>>, String> {
        let range = self
            .workbook
            .worksheet_range(sheet_name)
            .map_err(|e| format!("Failed to read sheet '{}': {}", sheet_name, e))?;

        let rows: Vec<Vec<Data>> = range
            .rows()
            .map(|row| row.to_vec())
            .collect();

        Ok(rows)
    }

    /// Convert a row to strings
    pub fn row_to_strings(row: &[Data]) -> Vec<String> {
        row.iter()
            .map(|cell| match cell {
                Data::Empty => String::new(),
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::DateTime(dt) => dt.to_string(),
                Data::DateTimeIso(s) => s.clone(),
                Data::DurationIso(s) => s.clone(),
                Data::Error(e) => format!("Error: {:?}", e),
            })
            .collect()
    }

    /// Get cell as string
    pub fn cell_to_string(cell: &Data) -> String {
        match cell {
            Data::Empty => String::new(),
            Data::String(s) => s.clone(),
            Data::Float(f) => f.to_string(),
            Data::Int(i) => i.to_string(),
            Data::Bool(b) => b.to_string(),
            Data::DateTime(dt) => dt.to_string(),
            Data::DateTimeIso(s) => s.clone(),
            Data::DurationIso(s) => s.clone(),
            Data::Error(e) => format!("Error: {:?}", e),
        }
    }

    /// Check if a cell is empty or contains only whitespace
    pub fn is_cell_empty(cell: &Data) -> bool {
        match cell {
            Data::Empty => true,
            Data::String(s) => s.trim().is_empty(),
            _ => false,
        }
    }

    /// Check if an entire row is empty
    pub fn is_row_empty(row: &[Data]) -> bool {
        row.iter().all(Self::is_cell_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_date_parser_string() {
        assert!(ExcelDateParser::parse_string("31-12-2024").is_some());
        assert!(ExcelDateParser::parse_string("2024-12-31").is_some());
        assert!(ExcelDateParser::parse_string("31 Dec 2024").is_some());
        assert!(ExcelDateParser::parse_string("16-Jan-2025").is_some());
    }

    #[test]
    fn test_excel_serial_date() {
        // 45658 should be 2024-12-31
        let date = ExcelDateParser::from_excel_serial(45658.0);
        assert!(date.is_some());
        let d = date.unwrap();
        assert_eq!(d.year(), 2024);
        assert_eq!(d.month(), 12);
        assert_eq!(d.day(), 31);
    }

    #[test]
    fn test_amount_parser() {
        assert_eq!(
            ExcelAmountParser::parse_string("1,234.56"),
            Some(Decimal::from_str("1234.56").unwrap())
        );
        assert_eq!(
            ExcelAmountParser::parse_string("Rs.100.00"),
            Some(Decimal::from_str("100.00").unwrap())
        );
        assert_eq!(
            ExcelAmountParser::parse_string("(50.00)"),
            Some(Decimal::from_str("-50.00").unwrap())
        );
    }

    #[test]
    fn test_column_detector() {
        let headers = vec![
            "Date".to_string(),
            "Description".to_string(),
            "Debit".to_string(),
            "Credit".to_string(),
            "Balance".to_string(),
        ];

        let mapping = ExcelColumnDetector::detect_columns(&headers);
        assert_eq!(mapping.date, Some(0));
        assert_eq!(mapping.description, Some(1));
        assert_eq!(mapping.debit, Some(2));
        assert_eq!(mapping.credit, Some(3));
        assert_eq!(mapping.balance, Some(4));
    }
}
