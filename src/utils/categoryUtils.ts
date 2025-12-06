// Category classification utilities

import categories from '../categories.json';
import { Currency } from '../types/data.types';

export type TransactionCategory =
  | 'Food'
  | 'Groceries'
  | 'Clothing'
  | 'Entertainment'
  | 'E-commerce'
  | 'Travel & Transport'
  | 'Utilities & Bills'
  | 'Healthcare'
  | 'Education'
  | 'Investments'
  | 'Transfers'
  | 'Bank Transfers'
  | 'Others';

/**
 * Convert any currency to INR for aggregation
 * Assumes 1 USD ≈ 83 INR (approximate exchange rate)
 */
export function convertToINR(amount: Currency): number {
  if (amount.currency === 'INR') {
    return amount.value;
  }
  // Convert USD to INR
  return amount.value * 83;
}

/**
 * Categorize a transaction or activity based on description
 * Uses keyword matching from categories.json + smart pattern detection
 */
export function categorizeTransaction(description: string): TransactionCategory {
  const lowerDesc = description.toLowerCase();

  // FIRST: Check each category's keywords (business/service categories)
  for (const [category, keywords] of Object.entries(categories)) {
    if (category === 'Others') continue; // Skip "Others" in first pass
    for (const keyword of keywords as string[]) {
      if (lowerDesc.includes(keyword.toLowerCase())) {
        return category as TransactionCategory;
      }
    }
  }

  // SECOND: Check for actual bank transfers (NEFT/IMPS/RTGS)
  const bankTransferPatterns = [
    /\bneft\b/i,
    /\bimps\b/i,
    /\brtgs\b/i,
    /to\s+\w*\s*bank\s+account/i,  // "to HDFC Bank Account"
    /from\s+\w*\s*bank\s+account/i,
  ];

  for (const pattern of bankTransferPatterns) {
    if (pattern.test(description)) {
      return 'Bank Transfers';
    }
  }

  // THIRD: Check for person-to-person transfers
  const transferPatterns = [
    /^sent\s+[₹$]/i,           // "Sent ₹500"
    /^received\s+[₹$]/i,       // "Received ₹500"
  ];

  for (const pattern of transferPatterns) {
    if (pattern.test(description)) {
      return 'Transfers';
    }
  }

  // FOURTH: Check if payment to a person (name pattern)
  // "Paid to [NAME]" where NAME is capitalized
  const personPaymentPattern = /(?:paid|to)\s+[A-Z][a-z]+(?:\s+[A-Z])?(?:\s+using\s+bank)/i;
  if (personPaymentPattern.test(description)) {
    return 'Transfers';
  }

  return 'Others'; // Default fallback
}

/**
 * Get category statistics for a list of transactions
 */
export function getCategoryStats(
  items: Array<{ description: string; amount: Currency }>
): Map<TransactionCategory, { count: number; total: number }> {
  const stats = new Map<TransactionCategory, { count: number; total: number }>();

  items.forEach(item => {
    const category = categorizeTransaction(item.description);
    const existing = stats.get(category) || { count: 0, total: 0 };

    existing.count++;
    existing.total += convertToINR(item.amount);

    stats.set(category, existing);
  });

  return stats;
}
