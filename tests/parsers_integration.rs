//! Integration tests for bank statement parsers

use finn_lens::parsers::{ParserOptions, ParserRegistry};

#[test]
fn test_icici_xls_parsing() {
    let registry = ParserRegistry::new();

    // Read the ICICI XLS sample file
    let data = std::fs::read("sample-data/icici_Statement-oFGl-1769-m8e6-1755-QhZl-6986.xls")
        .expect("Failed to read ICICI sample file");
    let options = ParserOptions::default();

    let result = registry
        .auto_parse("icici_Statement.xls", &data, &options)
        .expect("Failed to parse ICICI XLS file");

    assert!(result.bank_name.is_some());
    assert_eq!(result.bank_name.as_deref(), Some("ICICI Bank"));
    assert!(!result.transactions.is_empty(), "Should have parsed transactions");
    println!("ICICI: Parsed {} transactions", result.transactions.len());

    // Validate transaction structure
    for tx in &result.transactions {
        assert!(!tx.description.is_empty(), "Transaction should have description");
        assert!(!tx.amount.is_zero(), "Transaction should have non-zero amount");
    }
}

#[test]
fn test_idfc_first_xlsx_parsing() {
    let registry = ParserRegistry::new();

    // Read the IDFC First XLSX sample file
    let data = std::fs::read("sample-data/IDFCFIRSTBankstatement_10083875165.xlsx")
        .expect("Failed to read IDFC First sample file");
    let options = ParserOptions::default();

    let result = registry
        .auto_parse("IDFCFIRSTBankstatement_10083875165.xlsx", &data, &options)
        .expect("Failed to parse IDFC First XLSX file");

    assert!(result.bank_name.is_some());
    assert_eq!(result.bank_name.as_deref(), Some("IDFC First Bank"));
    assert!(!result.transactions.is_empty(), "Should have parsed transactions");
    println!("IDFC First: Parsed {} transactions", result.transactions.len());

    // Validate transaction structure
    for tx in &result.transactions {
        assert!(!tx.description.is_empty(), "Transaction should have description");
        assert!(!tx.amount.is_zero(), "Transaction should have non-zero amount");
    }
}
