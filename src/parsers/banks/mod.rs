//! Bank-specific parser implementations
//!
//! This module contains implementations for various banks, each with their
//! own parsing logic for different statement formats (Excel, etc.).

pub mod base;

pub use base::{AsAny, Bank, BankInfo, DetectionPattern, DetectionResult, FileFormat, FormatParser};

pub mod icici;
pub mod idfc_first;

pub use icici::ICICIBank;
pub use idfc_first::IDFCFirstBank;
