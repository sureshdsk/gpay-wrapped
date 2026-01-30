use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Error types for parsing operations
#[derive(Debug)]
pub enum ParserError {
    FileNotFound(String),
    UnsupportedFormat(String),
    ParseError(String),
    IoError(std::io::Error),
    Other(String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::FileNotFound(path) => write!(f, "File not found: {}", path),
            ParserError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
            ParserError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ParserError::IoError(e) => write!(f, "IO error: {}", e),
            ParserError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ParserError {}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> Self {
        ParserError::IoError(err)
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

/// A parsed transaction from a bank statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTransaction {
    pub date: NaiveDate,
    pub description: String,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub balance: Option<Decimal>,
    pub reference: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Credit => write!(f, "credit"),
            TransactionType::Debit => write!(f, "debit"),
        }
    }
}

/// Result of parsing a statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub transactions: Vec<ParsedTransaction>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub account_number: Option<String>,
    pub bank_name: Option<String>,
}

impl ParseResult {
    pub fn new(transactions: Vec<ParsedTransaction>) -> Self {
        let start_date = transactions.iter().map(|t| t.date).min();
        let end_date = transactions.iter().map(|t| t.date).max();

        Self {
            transactions,
            start_date,
            end_date,
            account_number: None,
            bank_name: None,
        }
    }
}

/// Options for parsing a statement
#[derive(Debug, Clone, Default)]
pub struct ParserOptions {
    pub date_format: Option<String>,
    pub skip_rows: usize,
}

/// Base trait for all statement parsers
pub trait Parser: Send + Sync {
    /// Returns the parser name/identifier
    fn name(&self) -> &str;

    /// Returns supported file extensions
    fn supported_extensions(&self) -> &[&str];

    /// Check if this parser can handle the given file
    fn can_parse(&self, file_path: &str) -> bool {
        let ext = file_path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();
        self.supported_extensions().contains(&ext.as_str())
    }

    /// Parse a statement file and return transactions
    fn parse(&self, file_path: &str, options: &ParserOptions) -> ParserResult<ParseResult>;

    /// Parse from bytes (for uploaded files)
    fn parse_bytes(&self, data: &[u8], options: &ParserOptions) -> ParserResult<ParseResult>;
}
