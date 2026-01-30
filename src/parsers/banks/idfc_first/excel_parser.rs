//! IDFC First Bank Excel statement parser
//!
//! IDFC First Bank XLSX exports have the following structure:
//! - Header row: Row 20 (0-indexed: row 19)
//! - Columns: Transaction Date, Value Date, Particulars, Cheque No., Debit, Credit, Balance
//! - Data starts: Row 21 (0-indexed: row 20)
//! - Date format: DD-Mon-YYYY (e.g., "16-Jan-2025")
//! - Metadata rows: 0-18
//! - Summary rows at end (after empty rows or "Total" marker)

use crate::parsers::banks::base::{FileFormat, FormatParser};
use crate::parsers::base::{ParseResult, ParsedTransaction, ParserError, ParserOptions, ParserResult, TransactionType};
use crate::parsers::formats::excel_base::{ExcelAmountParser, ExcelDateParser, ExcelReader};
use calamine::Data;

/// IDFC First Bank XLSX column indices (0-indexed)
struct IdfcFirstColumns {
    /// Transaction Date column
    transaction_date: usize,
    /// Value Date column
    _value_date: usize,
    /// Particulars column
    particulars: usize,
    /// Cheque No. column
    cheque_no: usize,
    /// Debit column
    debit: usize,
    /// Credit column
    credit: usize,
    /// Balance column
    balance: usize,
}

impl Default for IdfcFirstColumns {
    fn default() -> Self {
        Self {
            transaction_date: 0,
            _value_date: 1,
            particulars: 2,
            cheque_no: 3,
            debit: 4,
            credit: 5,
            balance: 6,
        }
    }
}

/// IDFC First Bank Excel parser
pub struct IdfcFirstExcelParser {
    columns: IdfcFirstColumns,
    header_row: usize,
    data_start_row: usize,
}

impl IdfcFirstExcelParser {
    pub fn new() -> Self {
        Self {
            columns: IdfcFirstColumns::default(),
            header_row: 19,      // 0-indexed: row 20
            data_start_row: 20,  // 0-indexed: row 21
        }
    }

    fn parse_excel_content(&self, data: &[u8], _options: &ParserOptions) -> ParserResult<ParseResult> {
        let mut reader = ExcelReader::from_bytes(data)
            .map_err(|e| ParserError::ParseError(e))?;

        let rows = reader.get_rows()
            .map_err(|e| ParserError::ParseError(e))?;

        if rows.len() <= self.data_start_row {
            // Try to find header row dynamically
            return self.parse_with_dynamic_header(data);
        }

        // Validate header row
        if rows.len() > self.header_row {
            let header_row = &rows[self.header_row];
            let header_text: String = header_row.iter()
                .map(|c| ExcelReader::cell_to_string(c).to_lowercase())
                .collect::<Vec<_>>()
                .join(" ");

            if !header_text.contains("transaction date") && !header_text.contains("particulars") {
                // Try to find header row by looking for column names
                return self.parse_with_dynamic_header(data);
            }
        }

        self.parse_rows(&rows, self.data_start_row)
    }

    fn parse_with_dynamic_header(&self, data: &[u8]) -> ParserResult<ParseResult> {
        let mut reader = ExcelReader::from_bytes(data)
            .map_err(|e| ParserError::ParseError(e))?;

        let rows = reader.get_rows()
            .map_err(|e| ParserError::ParseError(e))?;

        // Search for header row
        for (i, row) in rows.iter().enumerate() {
            let row_text: String = row.iter()
                .map(|c| ExcelReader::cell_to_string(c).to_lowercase())
                .collect::<Vec<_>>()
                .join(" ");

            if row_text.contains("transaction date") ||
               (row_text.contains("particulars") && row_text.contains("debit") && row_text.contains("credit")) {
                // Header found, data starts next row
                return self.parse_rows(&rows, i + 1);
            }
        }

        Err(ParserError::ParseError(
            "Could not find IDFC First header row".to_string(),
        ))
    }

    fn parse_rows(&self, rows: &[Vec<Data>], start_row: usize) -> ParserResult<ParseResult> {
        let mut transactions = Vec::new();
        let mut consecutive_empty_rows = 0;

        for row in rows.iter().skip(start_row) {
            // Skip empty rows, but track them
            if ExcelReader::is_row_empty(row) {
                consecutive_empty_rows += 1;
                // Stop after 3 consecutive empty rows (likely end of data)
                if consecutive_empty_rows >= 3 {
                    break;
                }
                continue;
            }
            consecutive_empty_rows = 0;

            // Stop at summary/total section
            if let Some(first_cell) = row.first() {
                let text = ExcelReader::cell_to_string(first_cell).to_lowercase();
                if text.contains("total") || text.contains("opening balance") ||
                   text.contains("closing balance") || text.contains("summary") {
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

            // Parse date from transaction date column
            let date = get_cell(self.columns.transaction_date)
                .and_then(ExcelDateParser::parse_cell);

            let date = match date {
                Some(d) => d,
                None => continue, // Skip rows without valid date
            };

            // Parse description/particulars
            let description = get_cell(self.columns.particulars)
                .map(|c| ExcelReader::cell_to_string(c).trim().to_string())
                .unwrap_or_default();

            // Skip empty descriptions
            if description.is_empty() {
                continue;
            }

            // Parse debit and credit amounts
            let debit = get_cell(self.columns.debit)
                .and_then(ExcelAmountParser::parse_cell);

            let credit = get_cell(self.columns.credit)
                .and_then(ExcelAmountParser::parse_cell);

            // Determine amount and type
            let (amount, tx_type) = match (debit, credit) {
                (Some(d), _) if !d.is_zero() => (d.abs(), TransactionType::Debit),
                (_, Some(c)) if !c.is_zero() => (c.abs(), TransactionType::Credit),
                _ => continue, // Skip rows without amounts
            };

            // Parse balance
            let balance = get_cell(self.columns.balance)
                .and_then(ExcelAmountParser::parse_cell);

            // Parse cheque number as reference
            let reference = get_cell(self.columns.cheque_no)
                .map(|c| ExcelReader::cell_to_string(c).trim().to_string())
                .filter(|s| !s.is_empty() && s != "0" && s != "-");

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
        result.bank_name = Some("IDFC First Bank".to_string());

        Ok(result)
    }
}

impl Default for IdfcFirstExcelParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatParser for IdfcFirstExcelParser {
    fn format(&self) -> FileFormat {
        FileFormat::Excel
    }

    fn bank_code(&self) -> &str {
        "idfc_first"
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
    fn test_idfc_first_excel_parser_creation() {
        let parser = IdfcFirstExcelParser::new();
        assert_eq!(parser.format(), FileFormat::Excel);
        assert_eq!(parser.bank_code(), "idfc_first");
    }

    #[test]
    fn test_can_parse() {
        let parser = IdfcFirstExcelParser::new();
        assert!(parser.can_parse("statement.xls", None));
        assert!(parser.can_parse("statement.xlsx", None));
        assert!(parser.can_parse("STATEMENT.XLSX", None));
        assert!(!parser.can_parse("statement.pdf", None));
        assert!(!parser.can_parse("statement.csv", None));
    }
}
