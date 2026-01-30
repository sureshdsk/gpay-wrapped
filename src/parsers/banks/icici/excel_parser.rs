//! ICICI Bank Excel statement parser
//!
//! ICICI Bank XLS exports have the following structure:
//! - Header row: Row 11 (0-indexed: row 10)
//! - Columns: S No., Value Date, Transaction Date, Cheque Number, Transaction Remarks,
//!            Withdrawal Amount(INR), Deposit Amount(INR), Balance(INR)
//! - Data starts: Row 12 (0-indexed: row 11)
//! - Date format: DD/MM/YYYY
//! - First column is often empty
//! - Metadata rows: 0-10
//! - Legend rows at the end (after "Legend" marker)

use crate::parsers::banks::base::{FileFormat, FormatParser};
use crate::parsers::base::{ParseResult, ParsedTransaction, ParserError, ParserOptions, ParserResult, TransactionType};
use crate::parsers::formats::excel_base::{ExcelAmountParser, ExcelDateParser, ExcelReader};
use calamine::Data;

/// ICICI Bank XLS column indices (0-indexed, accounting for empty first column)
struct IciciColumns {
    /// S No. column (index 0, often empty)
    _serial: usize,
    /// Value Date column
    value_date: usize,
    /// Transaction Date column
    _transaction_date: usize,
    /// Cheque Number column
    cheque_number: usize,
    /// Transaction Remarks column
    remarks: usize,
    /// Withdrawal Amount(INR) column
    withdrawal: usize,
    /// Deposit Amount(INR) column
    deposit: usize,
    /// Balance(INR) column
    balance: usize,
}

impl Default for IciciColumns {
    fn default() -> Self {
        Self {
            _serial: 0,
            value_date: 1,
            _transaction_date: 2,
            cheque_number: 3,
            remarks: 4,
            withdrawal: 5,
            deposit: 6,
            balance: 7,
        }
    }
}

/// ICICI Bank Excel parser
pub struct IciciExcelParser {
    columns: IciciColumns,
    header_row: usize,
    data_start_row: usize,
}

impl IciciExcelParser {
    pub fn new() -> Self {
        Self {
            columns: IciciColumns::default(),
            header_row: 10,      // 0-indexed: row 11
            data_start_row: 11,  // 0-indexed: row 12
        }
    }

    fn parse_excel_content(&self, data: &[u8], _options: &ParserOptions) -> ParserResult<ParseResult> {
        let mut reader = ExcelReader::from_bytes(data)
            .map_err(|e| ParserError::ParseError(e))?;

        let rows = reader.get_rows()
            .map_err(|e| ParserError::ParseError(e))?;

        if rows.len() <= self.data_start_row {
            return Err(ParserError::ParseError(
                "File does not have enough rows for ICICI format".to_string(),
            ));
        }

        // Validate header row
        if rows.len() > self.header_row {
            let header_row = &rows[self.header_row];
            let header_text: String = header_row.iter()
                .map(|c| ExcelReader::cell_to_string(c).to_lowercase())
                .collect::<Vec<_>>()
                .join(" ");

            if !header_text.contains("value date") && !header_text.contains("transaction") {
                // Try to find header row by looking for "Value Date" or "Transaction Date"
                for (i, row) in rows.iter().enumerate() {
                    let row_text: String = row.iter()
                        .map(|c| ExcelReader::cell_to_string(c).to_lowercase())
                        .collect::<Vec<_>>()
                        .join(" ");
                    if row_text.contains("value date") || row_text.contains("transaction date") {
                        // Header found, data starts next row
                        return self.parse_with_custom_start(data, i + 1);
                    }
                }
                // If we get here, no header was found
                return Err(ParserError::ParseError(
                    "Could not find ICICI header row".to_string(),
                ));
            }
        }

        self.parse_rows(&rows, self.data_start_row)
    }

    fn parse_with_custom_start(&self, data: &[u8], start_row: usize) -> ParserResult<ParseResult> {
        let mut reader = ExcelReader::from_bytes(data)
            .map_err(|e| ParserError::ParseError(e))?;

        let rows = reader.get_rows()
            .map_err(|e| ParserError::ParseError(e))?;

        self.parse_rows(&rows, start_row)
    }

    fn parse_rows(&self, rows: &[Vec<Data>], start_row: usize) -> ParserResult<ParseResult> {
        let mut transactions = Vec::new();

        for row in rows.iter().skip(start_row) {
            // Skip empty rows
            if ExcelReader::is_row_empty(row) {
                continue;
            }

            // Stop at legend section
            if let Some(first_cell) = row.first() {
                let text = ExcelReader::cell_to_string(first_cell).to_lowercase();
                if text.contains("legend") || text.contains("note:") || text.contains("*") && text.len() < 10 {
                    break;
                }
            }

            // Get cell values safely
            let get_cell = |idx: usize| -> Option<&Data> {
                if idx < row.len() {
                    Some(&row[idx])
                } else {
                    None
                }
            };

            // Parse date from value date column
            let date = get_cell(self.columns.value_date)
                .and_then(ExcelDateParser::parse_cell);

            let date = match date {
                Some(d) => d,
                None => continue, // Skip rows without valid date
            };

            // Parse description/remarks
            let description = get_cell(self.columns.remarks)
                .map(|c| ExcelReader::cell_to_string(c).trim().to_string())
                .unwrap_or_default();

            // Skip empty descriptions
            if description.is_empty() {
                continue;
            }

            // Parse withdrawal and deposit amounts
            let withdrawal = get_cell(self.columns.withdrawal)
                .and_then(ExcelAmountParser::parse_cell);

            let deposit = get_cell(self.columns.deposit)
                .and_then(ExcelAmountParser::parse_cell);

            // Determine amount and type
            let (amount, tx_type) = match (withdrawal, deposit) {
                (Some(w), _) if !w.is_zero() => (w.abs(), TransactionType::Debit),
                (_, Some(d)) if !d.is_zero() => (d.abs(), TransactionType::Credit),
                _ => continue, // Skip rows without amounts
            };

            // Parse balance
            let balance = get_cell(self.columns.balance)
                .and_then(ExcelAmountParser::parse_cell);

            // Parse cheque number as reference
            let reference = get_cell(self.columns.cheque_number)
                .map(|c| ExcelReader::cell_to_string(c).trim().to_string())
                .filter(|s| !s.is_empty() && s != "0");

            transactions.push(ParsedTransaction {
                date,
                description,
                amount,
                transaction_type: tx_type,
                balance,
                reference,
                mode: None,
            });
        }

        let mut result = ParseResult::new(transactions);
        result.bank_name = Some("ICICI Bank".to_string());

        Ok(result)
    }
}

impl Default for IciciExcelParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for IciciExcelParser {
    fn format(&self) -> FileFormat {
        FileFormat::Excel
    }

    fn bank_code(&self) -> &str {
        "icici"
    }

    fn can_parse(&self, file_path: &str, _content: Option<&[u8]>) -> bool {
        if let Some(ext) = file_path.rsplit('.').next() {
            let ext_lower = ext.to_lowercase();
            return ext_lower == "xls" || ext_lower == "xlsx";
        }
        false
    }

    fn parse(&self, file_path: &str, options: &ParserOptions) -> ParserResult<ParseResult> {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            return Err(ParserError::FileNotFound(file_path.to_string()));
        }

        let data = std::fs::read(path)?;
        self.parse_excel_content(&data, options)
    }

    fn parse_bytes(&self, data: &[u8], options: &ParserOptions) -> ParserResult<ParseResult> {
        self.parse_excel_content(data, options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icici_excel_parser_creation() {
        let parser = IciciExcelParser::new();
        assert_eq!(parser.format(), FileFormat::Excel);
        assert_eq!(parser.bank_code(), "icici");
    }

    #[test]
    fn test_can_parse() {
        let parser = IciciExcelParser::new();
        assert!(parser.can_parse("statement.xls", None));
        assert!(parser.can_parse("statement.xlsx", None));
        assert!(parser.can_parse("STATEMENT.XLS", None));
        assert!(!parser.can_parse("statement.pdf", None));
        assert!(!parser.can_parse("statement.csv", None));
    }
}
