import { api } from './client'
import type { Transaction, TransactionFilters } from '@/types'

interface CreateTransactionParams {
  accountId: number
  categoryId?: number
  transactionDate: string
  postedDate?: string
  description: string
  originalDescription?: string
  amount: string
  transactionType: 'debit' | 'credit'
  merchantName?: string
  referenceNumber?: string
  notes?: string
}

interface UpdateTransactionParams {
  categoryId?: number
  description?: string
  merchantName?: string
  notes?: string
  isRecurring?: boolean
  isExcluded?: boolean
}

// API response types (snake_case from backend)
interface ApiTransaction {
  id: number
  pid: string
  account_id: number
  category_id: number | null
  transaction_date: string
  posted_date: string | null
  description: string
  original_description: string | null
  amount: string
  transaction_type: 'debit' | 'credit'
  status: 'pending' | 'posted' | 'cancelled'
  merchant_name: string | null
  reference_number: string | null
  notes: string | null
  is_recurring: boolean
  is_excluded: boolean
  created_at: string
}

interface ApiTransactionListResponse {
  transactions: ApiTransaction[]
  page: number
  per_page: number
}

function transformTransaction(t: ApiTransaction): Transaction {
  return {
    id: t.id,
    pid: t.pid,
    accountId: t.account_id,
    categoryId: t.category_id,
    transactionDate: t.transaction_date,
    postedDate: t.posted_date,
    description: t.description,
    originalDescription: t.original_description,
    amount: t.amount,
    transactionType: t.transaction_type,
    status: t.status,
    merchantName: t.merchant_name,
    referenceNumber: t.reference_number,
    notes: t.notes,
    isRecurring: t.is_recurring,
    isExcluded: t.is_excluded,
    createdAt: t.created_at,
  }
}

export interface TransactionListResponse {
  transactions: Transaction[]
  page: number
  perPage: number
}

export const transactionsApi = {
  list: async (filters?: TransactionFilters): Promise<TransactionListResponse> => {
    const data = await api.get<ApiTransactionListResponse>('/v1/transactions', {
      account_id: filters?.accountId,
      category_id: filters?.categoryId,
      start_date: filters?.startDate,
      end_date: filters?.endDate,
      transaction_type: filters?.transactionType,
      search: filters?.search,
      min_amount: filters?.minAmount,
      max_amount: filters?.maxAmount,
      page: filters?.page,
      per_page: filters?.perPage,
    })
    return {
      transactions: data.transactions.map(transformTransaction),
      page: data.page,
      perPage: data.per_page,
    }
  },

  recent: async (): Promise<Transaction[]> => {
    const data = await api.get<ApiTransaction[]>('/v1/transactions/recent')
    return data.map(transformTransaction)
  },

  get: async (pid: string): Promise<Transaction> => {
    const data = await api.get<ApiTransaction>(`/v1/transactions/${pid}`)
    return transformTransaction(data)
  },

  create: async (params: CreateTransactionParams): Promise<Transaction> => {
    const data = await api.post<ApiTransaction>('/v1/transactions', {
      account_id: params.accountId,
      category_id: params.categoryId,
      transaction_date: params.transactionDate,
      posted_date: params.postedDate,
      description: params.description,
      original_description: params.originalDescription,
      amount: params.amount,
      transaction_type: params.transactionType,
      merchant_name: params.merchantName,
      reference_number: params.referenceNumber,
      notes: params.notes,
    })
    return transformTransaction(data)
  },

  update: async (pid: string, params: UpdateTransactionParams): Promise<Transaction> => {
    const data = await api.put<ApiTransaction>(`/v1/transactions/${pid}`, {
      category_id: params.categoryId,
      description: params.description,
      merchant_name: params.merchantName,
      notes: params.notes,
      is_recurring: params.isRecurring,
      is_excluded: params.isExcluded,
    })
    return transformTransaction(data)
  },

  delete: (pid: string) =>
    api.delete<{ status: string }>(`/v1/transactions/${pid}`),
}
