import { api } from './client'
import type { Statement } from '@/types'

export interface TransactionPreview {
  date: string
  description: string
  amount: string
  transaction_type: string
  balance: string | null
}

export interface UploadResponse {
  statement: Statement
  preview: TransactionPreview[]
}

export interface ConfirmImportResponse {
  status: string
  transactions_created: number
  statement_id: string
}

export interface ParsersResponse {
  parsers: string[]
  supported_extensions: string[]
}

export const statementsApi = {
  list: () => api.get<Statement[]>('/v1/statements'),

  get: (pid: string) => api.get<Statement>(`/v1/statements/${pid}`),

  upload: async (file: File, accountId?: number): Promise<UploadResponse> => {
    const formData = new FormData()
    formData.append('file', file)
    if (accountId) {
      formData.append('account_id', accountId.toString())
    }

    const token = localStorage.getItem('auth_token')
    const response = await fetch('/api/v1/statements/upload', {
      method: 'POST',
      headers: {
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
      },
      body: formData,
    })

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Upload failed' }))
      throw new Error(error.message || error.error || 'Upload failed')
    }

    return response.json()
  },

  confirmImport: (pid: string, accountId: number) =>
    api.post<ConfirmImportResponse>(`/v1/statements/${pid}/confirm`, {
      account_id: accountId,
    }),

  delete: (pid: string) =>
    api.delete<{ status: string }>(`/v1/statements/${pid}`),

  getParsers: () => api.get<ParsersResponse>('/v1/statements/parsers'),
}
