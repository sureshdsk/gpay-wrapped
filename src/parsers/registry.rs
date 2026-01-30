use super::base::{ParseResult, ParserError, ParserOptions, ParserResult};
use super::banks::{Bank, FileFormat};
use super::detector::BankDetector;
use super::banks::ICICIBank;
use super::banks::IDFCFirstBank;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of available parsers with bank support
pub struct ParserRegistry {
    /// Bank-specific implementations
    banks: HashMap<String, Arc<dyn Bank>>,

    /// Bank detector for automatic identification
    detector: BankDetector,
}

impl ParserRegistry {
    /// Create a new registry with default banks
    pub fn new() -> Self {
        let mut registry = Self {
            banks: HashMap::new(),
            detector: BankDetector::new(),
        };

        // Register default banks
        registry.register_bank(Arc::new(ICICIBank::new()));
        registry.register_bank(Arc::new(IDFCFirstBank::new()));

        registry
    }

    /// Register a bank
    pub fn register_bank(&mut self, bank: Arc<dyn Bank>) {
        let code = bank.info().code.to_string();
        // Also register with detector
        self.detector.register_bank(bank.clone());
        self.banks.insert(code, bank);
    }

    /// Get a bank by code
    pub fn get_bank(&self, code: &str) -> Option<Arc<dyn Bank>> {
        self.banks.get(code).cloned()
    }

    /// Auto-detect bank and parse a statement file
    ///
    /// This is the preferred method for parsing statements. It will:
    /// 1. Detect the bank from filename and content
    /// 2. Select the appropriate parser for the file format
    /// 3. Parse the statement using bank-specific logic
    pub fn auto_parse(
        &self,
        file_path: &str,
        data: &[u8],
        options: &ParserOptions,
    ) -> ParserResult<ParseResult> {
        // Try to detect bank
        match self.detector.detect(file_path, data) {
            Ok(detection) => {
                // Get the bank
                if let Some(bank) = self.get_bank(&detection.bank) {
                    // Get the format-specific parser
                    if let Some(parser) = bank.get_parser(detection.format) {
                        let mut result = parser.parse_bytes(data, options)?;

                        // Store detection information
                        result.bank_name = Some(bank.info().name.to_string());

                        return Ok(result);
                    }
                }
            }
            Err(_) => {
                // Detection failed, fall through to extension-based parsing
            }
        }

        // Fallback to extension-based parsing
        self.parse_by_extension(file_path, data, options)
    }

    /// Parse with explicit bank and format selection
    ///
    /// Use this when you know the bank and format upfront, or when
    /// you want to override automatic detection.
    pub fn parse_with_bank(
        &self,
        bank_code: &str,
        format: FileFormat,
        data: &[u8],
        options: &ParserOptions,
    ) -> ParserResult<ParseResult> {
        let bank = self
            .get_bank(bank_code)
            .ok_or_else(|| ParserError::ParseError(format!("Bank not found: {}", bank_code)))?;

        let parser = bank
            .get_parser(format)
            .ok_or_else(|| {
                ParserError::ParseError(format!(
                    "No parser for format {} in bank {}",
                    format.as_str(),
                    bank_code
                ))
            })?;

        let mut result = parser.parse_bytes(data, options)?;
        result.bank_name = Some(bank.info().name.to_string());

        Ok(result)
    }

    /// Parse by file extension (fallback method)
    fn parse_by_extension(
        &self,
        file_path: &str,
        data: &[u8],
        options: &ParserOptions,
    ) -> ParserResult<ParseResult> {
        // Determine format from extension
        let ext = file_path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        let format = FileFormat::from_extension(&ext)
            .ok_or_else(|| ParserError::UnsupportedFormat(format!("Unknown extension: {}", ext)))?;

        // Try each bank's parser for this format
        for bank in self.banks.values() {
            if let Some(parser) = bank.get_parser(format) {
                if parser.can_parse(file_path, Some(data)) {
                    match parser.parse_bytes(data, options) {
                        Ok(mut result) => {
                            result.bank_name = Some(bank.info().name.to_string());
                            return Ok(result);
                        }
                        Err(_) => continue, // Try next bank
                    }
                }
            }
        }

        Err(ParserError::UnsupportedFormat(format!(
            "No parser found for: {}",
            file_path
        )))
    }

    /// List all registered banks
    pub fn list_banks(&self) -> Vec<&str> {
        self.banks.keys().map(|s| s.as_str()).collect()
    }

    /// List available parsers (returns format names)
    pub fn list(&self) -> Vec<&str> {
        vec!["excel"]
    }

    /// Get bank info for all registered banks
    pub fn bank_info(&self) -> Vec<&str> {
        self.banks
            .values()
            .map(|b| b.info().name)
            .collect()
    }

    /// Get supported extensions
    pub fn supported_extensions(&self) -> Vec<String> {
        vec!["xls".to_string(), "xlsx".to_string()]
    }

    /// Get available parsers for a specific bank
    pub fn get_bank_parsers(&self, bank_code: &str) -> Option<Vec<String>> {
        self.get_bank(bank_code).map(|bank| {
            bank.parsers()
                .iter()
                .map(|p| p.name())
                .collect()
        })
    }

    /// Get reference to the bank detector (for accessing detection info)
    pub fn detector(&self) -> &BankDetector {
        &self.detector
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ParserRegistry::new();
        assert!(!registry.list_banks().is_empty());
    }

    #[test]
    fn test_registry_banks() {
        let registry = ParserRegistry::new();
        assert!(registry.get_bank("icici").is_some());
        assert!(registry.get_bank("idfc_first").is_some());
        assert!(registry.get_bank("sbi").is_none());
    }

    #[test]
    fn test_list_banks() {
        let registry = ParserRegistry::new();
        let banks = registry.list_banks();
        assert!(banks.contains(&"icici"));
        assert!(banks.contains(&"idfc_first"));
    }

    #[test]
    fn test_supported_extensions() {
        let registry = ParserRegistry::new();
        let extensions = registry.supported_extensions();
        assert!(extensions.contains(&"xls".to_string()));
        assert!(extensions.contains(&"xlsx".to_string()));
    }

    #[test]
    fn test_get_bank_parsers() {
        let registry = ParserRegistry::new();
        let icici_parsers = registry.get_bank_parsers("icici");
        assert!(icici_parsers.is_some());
        let parsers = icici_parsers.unwrap();
        assert!(parsers.iter().any(|p| p.contains("excel")));
    }
}
