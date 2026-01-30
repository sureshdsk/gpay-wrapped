// User types
export interface User {
  id: number
  pid: string
  email: string
  name: string
  createdAt: string
}

export interface AuthResponse {
  token: string
  pid: string
  name: string
  is_verified: boolean
}

// Feature types
export interface Feature {
  id: number
  key: string
  name: string
  description: string | null
  category: string
  isPremium: boolean
  sortOrder: number
}

export interface UserFeature extends Feature {
  enabled: boolean
}

// Account types
export interface Account {
  id: number
  pid: string
  name: string
  accountType: 'checking' | 'savings' | 'credit_card' | 'investment'
  institution: string | null
  accountNumberLast4: string | null
  currency: string
  currentBalance: string
  availableBalance: string | null
  color: string
  isActive: boolean
  lastSyncedAt: string | null
  createdAt: string
}

export interface AccountsSummary {
  totalBalance: string
  accounts: Account[]
}

// Category types
export interface Category {
  id: number
  name: string
  color: string
  icon: string | null
  categoryType: 'income' | 'expense'
  isSystem: boolean
}

// Transaction types
export interface Transaction {
  id: number
  pid: string
  accountId: number
  categoryId: number | null
  transactionDate: string
  postedDate: string | null
  description: string
  originalDescription: string | null
  amount: string
  transactionType: 'debit' | 'credit'
  status: 'pending' | 'posted' | 'cancelled'
  merchantName: string | null
  referenceNumber: string | null
  notes: string | null
  isRecurring: boolean
  isExcluded: boolean
  createdAt: string
}

export interface TransactionFilters {
  accountId?: number
  categoryId?: number
  startDate?: string
  endDate?: string
  transactionType?: 'debit' | 'credit'
  search?: string
  minAmount?: string
  maxAmount?: string
  page?: number
  perPage?: number
}

export interface TransactionListResponse {
  transactions: Transaction[]
  page: number
  perPage: number
}

// Statement types
export interface Statement {
  id: number
  pid: string
  accountId: number | null
  filename: string
  fileType: string
  fileSize: number
  status: 'pending' | 'processing' | 'completed' | 'failed'
  statementDate: string | null
  startDate: string | null
  endDate: string | null
  transactionCount: number
  errorMessage: string | null
  processedAt: string | null
  createdAt: string
}

// API response types
export interface ApiError {
  error: string
  message: string
}

// Dashboard/Insights types
export interface RecentTransaction {
  id: number
  pid: string
  description: string
  amount: string
  transactionType: 'debit' | 'credit'
  transactionDate: string
  categoryName: string | null
  accountName: string
}

export interface MonthlySummary {
  month: string
  income: string
  expenses: string
  net: string
  transactionCount: number
}

export interface DashboardSummary {
  totalBalance: string
  accountCount: number
  recentTransactions: RecentTransaction[]
  thisMonth: MonthlySummary
}
