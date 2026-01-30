import { api } from './client'
import type { Account, AccountsSummary } from '@/types'

interface CreateAccountParams {
  name: string
  accountType: string
  institution?: string
  accountNumberLast4?: string
  currency?: string
  currentBalance: string
  availableBalance?: string
  color?: string
}

interface UpdateAccountParams {
  name?: string
  institution?: string
  color?: string
  isActive?: boolean
  currentBalance?: string
  availableBalance?: string
}

// API response types (snake_case from backend)
interface ApiAccount {
  id: number
  pid: string
  name: string
  account_type: string
  institution: string | null
  account_number_last4: string | null
  currency: string
  current_balance: string
  available_balance: string | null
  color: string
  is_active: boolean
  last_synced_at: string | null
  created_at: string
}

interface ApiAccountsSummary {
  total_balance: string
  accounts: ApiAccount[]
}

// Transform API response to frontend types
function transformAccount(apiAccount: ApiAccount): Account {
  return {
    id: apiAccount.id,
    pid: apiAccount.pid,
    name: apiAccount.name,
    accountType: apiAccount.account_type as Account['accountType'],
    institution: apiAccount.institution,
    accountNumberLast4: apiAccount.account_number_last4,
    currency: apiAccount.currency,
    currentBalance: apiAccount.current_balance,
    availableBalance: apiAccount.available_balance,
    color: apiAccount.color,
    isActive: apiAccount.is_active,
    lastSyncedAt: apiAccount.last_synced_at,
    createdAt: apiAccount.created_at,
  }
}

export const accountsApi = {
  list: async (): Promise<AccountsSummary> => {
    const data = await api.get<ApiAccountsSummary>('/v1/accounts')
    return {
      totalBalance: data.total_balance,
      accounts: data.accounts.map(transformAccount),
    }
  },

  get: async (pid: string): Promise<Account> => {
    const data = await api.get<ApiAccount>(`/v1/accounts/${pid}`)
    return transformAccount(data)
  },

  create: async (params: CreateAccountParams): Promise<Account> => {
    const data = await api.post<ApiAccount>('/v1/accounts', {
      name: params.name,
      account_type: params.accountType,
      institution: params.institution,
      account_number_last4: params.accountNumberLast4,
      currency: params.currency,
      current_balance: params.currentBalance,
      available_balance: params.availableBalance,
      color: params.color,
    })
    return transformAccount(data)
  },

  update: async (pid: string, params: UpdateAccountParams): Promise<Account> => {
    const data = await api.put<ApiAccount>(`/v1/accounts/${pid}`, {
      name: params.name,
      institution: params.institution,
      color: params.color,
      is_active: params.isActive,
      current_balance: params.currentBalance,
      available_balance: params.availableBalance,
    })
    return transformAccount(data)
  },

  delete: (pid: string) =>
    api.delete<{ status: string }>(`/v1/accounts/${pid}`),
}
