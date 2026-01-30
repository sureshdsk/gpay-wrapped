pub mod base;
pub mod registry;
pub mod detector;
pub mod banks;
pub mod formats;

pub use base::{ParsedTransaction, ParserError, ParserOptions, ParserResult, TransactionType};
pub use registry::ParserRegistry;
pub use detector::BankDetector;
pub use banks::{Bank, BankInfo, DetectionPattern, DetectionResult, FileFormat, FormatParser};
pub use banks::{ICICIBank, IDFCFirstBank};
