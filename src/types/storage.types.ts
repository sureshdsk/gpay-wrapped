// Zustand store types

import { ParsedData, RawExtractedData } from './data.types';
import { Insight } from './insight.types';

// Re-export types used by dataStore
export type { RawExtractedData } from './data.types';

export type YearFilter = '2025' | 'all';

export interface DataStore {
  // State
  rawData: RawExtractedData | null;
  parsedData: ParsedData | null;
  insights: Insight[];
  selectedYear: YearFilter;
  isLoading: boolean;
  error: string | null;

  // Actions
  setRawData: (data: RawExtractedData) => void;
  setParsedData: (data: ParsedData) => void;
  setInsights: (insights: Insight[]) => void;
  setSelectedYear: (year: YearFilter) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  parseRawData: () => void;
  recalculateInsights: (year: YearFilter) => void;
}
