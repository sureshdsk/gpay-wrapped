//! Format-specific base utilities
//!
//! This module contains generic utilities for different file formats (Excel, etc.)
//! that can be reused across bank-specific implementations.

pub mod excel_base;

pub use excel_base::{
    ExcelAmountParser, ExcelColumnDetector, ExcelColumnMapping, ExcelDateParser, ExcelReader,
};
