/**
 * Multi-Layer Classification Rule Engine Types
 * Supports priority-based, configurable merchant categorization
 */

export type RuleType = 'exclusion' | 'exact' | 'keyword' | 'pattern' | 'fuzzy' | 'heuristic';

export interface ClassificationRule {
  priority: number; // 1 = highest, 10 = lowest
  type: RuleType;
  matcher: string | RegExp;
  category: string;
  description?: string;
}

export interface ExclusionRule {
  type: 'payment_gateway' | 'bank_iso' | 'personal_transfer' | 'technical';
  patterns: string[];
  keywords: string[];
  description: string;
}

export interface ExactMatchRule {
  merchant: string; // Exact merchant name (case-insensitive)
  category: string;
  priority: number;
}

export interface KeywordRule {
  keyword: string;
  category: string;
  priority: number;
  caseSensitive?: boolean;
}

export interface PatternRule {
  pattern: string; // Regex pattern
  category: string;
  priority: number;
  flags?: string;
}

export interface FuzzyRule {
  baseKeyword: string;
  variants: string[]; // Misspellings, abbreviations
  category: string;
  priority: number;
}

export interface HeuristicRule {
  type: 'amount_based' | 'context_based' | 'name_pattern';
  condition: (merchant: string, amount?: number, context?: any) => boolean;
  category: string;
  priority: number;
  description: string;
}

export interface CategoryConfig {
  name: string;
  exactMatches?: string[];
  keywords: string[];
  patterns: string[];
  fuzzyKeywords?: Record<string, string[]>; // base -> variants
  priority?: number; // Default priority for rules in this category
}

export interface ClassifierConfig {
  version: string;
  exclusions: {
    payment_gateways: string[];
    bank_isos: string[];
    personal_indicators: string[];
    technical_terms: string[];
  };
  categories: Record<string, CategoryConfig>;
}

export interface ClassificationResult {
  category: string;
  confidence: number; // 0-1
  matchedRule: {
    type: RuleType;
    priority: number;
    matcher: string;
  };
  isExcluded?: boolean;
}

export interface Transaction {
  merchant: string;
  amount: number;
  date: string;
  category: string;
  rawDate: string;
}
