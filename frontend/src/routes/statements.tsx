import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { useState, useCallback, useEffect } from 'react'
import { statementsApi, type TransactionPreview, type UploadResponse } from '@/lib/api/statements'
import { accountsApi } from '@/lib/api/accounts'
import type { Statement, Account } from '@/types'
import { Upload, FileText, Check, X, Loader2, AlertCircle, Trash2, Plus, Wallet } from 'lucide-react'
import { cn } from '@/lib/utils'

export const Route = createFileRoute('/statements')({
  component: StatementsPage,
})

function StatementsPage() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)

  if (!isAuthenticated) {
    return <Navigate to="/login" />
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
          Statements
        </h2>
        <p className="text-slate-500 mt-1">Upload and manage your bank statements</p>
      </div>

      <UploadSection />
      <StatementsList />
    </div>
  )
}

const ACCOUNT_TYPES = [
  { value: 'checking', label: 'Checking' },
  { value: 'savings', label: 'Savings' },
  { value: 'credit_card', label: 'Credit Card' },
  { value: 'investment', label: 'Investment' },
]

const ACCOUNT_COLORS = [
  '#3B82F6', // blue
  '#10B981', // emerald
  '#F59E0B', // amber
  '#EF4444', // red
  '#8B5CF6', // violet
  '#EC4899', // pink
  '#06B6D4', // cyan
  '#84CC16', // lime
]

function UploadSection() {
  const [isDragging, setIsDragging] = useState(false)
  const [file, setFile] = useState<File | null>(null)
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [uploadResult, setUploadResult] = useState<UploadResponse | null>(null)
  const [accounts, setAccounts] = useState<Account[]>([])
  const [selectedAccountId, setSelectedAccountId] = useState<number | null>(null)
  const [isConfirming, setIsConfirming] = useState(false)
  const [importSuccess, setImportSuccess] = useState<{ count: number } | null>(null)

  // New account form state
  const [showNewAccountForm, setShowNewAccountForm] = useState(false)
  const [isCreatingAccount, setIsCreatingAccount] = useState(false)
  const [newAccountName, setNewAccountName] = useState('')
  const [newAccountType, setNewAccountType] = useState('checking')
  const [newAccountInstitution, setNewAccountInstitution] = useState('')
  const [newAccountColor, setNewAccountColor] = useState(ACCOUNT_COLORS[0])

  const loadAccounts = useCallback(async () => {
    try {
      const data = await accountsApi.list()
      setAccounts(data.accounts)
      if (data.accounts.length > 0 && !selectedAccountId) {
        setSelectedAccountId(data.accounts[0].id)
      }
      // Auto-show create form if no accounts
      if (data.accounts.length === 0) {
        setShowNewAccountForm(true)
      }
    } catch (err) {
      console.error('Failed to load accounts:', err)
    }
  }, [selectedAccountId])

  useEffect(() => {
    loadAccounts()
  }, [loadAccounts])

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(true)
  }, [])

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
  }, [])

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragging(false)
    const droppedFile = e.dataTransfer.files[0]
    if (droppedFile) {
      setFile(droppedFile)
      setError(null)
      setUploadResult(null)
      setImportSuccess(null)
    }
  }, [])

  const handleFileSelect = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0]
    if (selectedFile) {
      setFile(selectedFile)
      setError(null)
      setUploadResult(null)
      setImportSuccess(null)
    }
  }, [])

  const handleUpload = async () => {
    if (!file) return

    setIsUploading(true)
    setError(null)

    try {
      const result = await statementsApi.upload(file)
      setUploadResult(result)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Upload failed')
    } finally {
      setIsUploading(false)
    }
  }

  const handleCreateAccount = async () => {
    if (!newAccountName.trim()) {
      setError('Account name is required')
      return
    }

    setIsCreatingAccount(true)
    setError(null)

    try {
      const newAccount = await accountsApi.create({
        name: newAccountName.trim(),
        accountType: newAccountType,
        institution: newAccountInstitution.trim() || undefined,
        currentBalance: '0',
        color: newAccountColor,
      })

      // Add to accounts list and select it
      setAccounts(prev => [...prev, newAccount])
      setSelectedAccountId(newAccount.id)

      // Reset form
      setShowNewAccountForm(false)
      setNewAccountName('')
      setNewAccountType('checking')
      setNewAccountInstitution('')
      setNewAccountColor(ACCOUNT_COLORS[0])
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create account')
    } finally {
      setIsCreatingAccount(false)
    }
  }

  const handleConfirmImport = async () => {
    if (!uploadResult || !selectedAccountId) return

    setIsConfirming(true)
    setError(null)

    try {
      const result = await statementsApi.confirmImport(
        uploadResult.statement.pid,
        selectedAccountId
      )
      setImportSuccess({ count: result.transactions_created })
      setUploadResult(null)
      setFile(null)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Import failed')
    } finally {
      setIsConfirming(false)
    }
  }

  const handleCancel = () => {
    setFile(null)
    setUploadResult(null)
    setError(null)
    setImportSuccess(null)
  }

  return (
    <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6 shadow-sm">
      <h3 className="text-lg font-bold mb-4">Upload Statement</h3>

      {importSuccess && (
        <div className="mb-4 p-4 bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 dark:border-emerald-800 rounded-lg flex items-center gap-3">
          <Check className="w-5 h-5 text-emerald-600" />
          <span className="text-emerald-700 dark:text-emerald-400 font-medium">
            Successfully imported {importSuccess.count} transactions!
          </span>
        </div>
      )}

      {error && (
        <div className="mb-4 p-4 bg-rose-50 dark:bg-rose-900/20 border border-rose-200 dark:border-rose-800 rounded-lg flex items-center gap-3">
          <AlertCircle className="w-5 h-5 text-rose-600" />
          <span className="text-rose-700 dark:text-rose-400 font-medium">{error}</span>
        </div>
      )}

      {!uploadResult ? (
        <>
          <div
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            className={cn(
              'border-2 border-dashed rounded-xl p-8 text-center transition-colors',
              isDragging
                ? 'border-primary bg-primary/5'
                : 'border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600',
              file && 'border-emerald-500 bg-emerald-50 dark:bg-emerald-900/20'
            )}
          >
            {file ? (
              <div className="flex items-center justify-center gap-3">
                <FileText className="w-8 h-8 text-emerald-600" />
                <div className="text-left">
                  <p className="font-semibold text-slate-900 dark:text-white">{file.name}</p>
                  <p className="text-sm text-slate-500">
                    {(file.size / 1024 / 1024).toFixed(2)} MB
                  </p>
                </div>
                <button
                  onClick={handleCancel}
                  className="ml-4 p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
                >
                  <X className="w-5 h-5 text-slate-500" />
                </button>
              </div>
            ) : (
              <>
                <Upload className="w-12 h-12 text-slate-400 mx-auto mb-4" />
                <p className="text-slate-600 dark:text-slate-400 mb-2">
                  Drag and drop your statement file here, or
                </p>
                <label className="inline-block">
                  <input
                    type="file"
                    accept=".xls,.xlsx"
                    onChange={handleFileSelect}
                    className="hidden"
                  />
                  <span className="text-primary font-semibold cursor-pointer hover:underline">
                    browse to upload
                  </span>
                </label>
                <p className="text-xs text-slate-400 mt-2">
                  Supported formats: Excel (.xls, .xlsx) - max 50MB
                </p>
              </>
            )}
          </div>

          {file && (
            <div className="mt-4">
              <button
                onClick={handleUpload}
                disabled={isUploading}
                className="w-full py-3 bg-primary text-white rounded-lg font-semibold hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
              >
                {isUploading ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" />
                    Uploading...
                  </>
                ) : (
                  <>
                    <Upload className="w-5 h-5" />
                    Upload & Parse
                  </>
                )}
              </button>
            </div>
          )}
        </>
      ) : (
        <div className="space-y-4">
          <div className="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <span className="font-semibold">{uploadResult.statement.filename}</span>
              <span className="text-sm text-emerald-600 font-medium">
                {uploadResult.preview.length} transactions found
              </span>
            </div>
            {uploadResult.statement.startDate && uploadResult.statement.endDate && (
              <p className="text-sm text-slate-500">
                Period: {uploadResult.statement.startDate} to {uploadResult.statement.endDate}
              </p>
            )}
          </div>

          {/* Account Selection */}
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
              Import to Account
            </label>

            {!showNewAccountForm ? (
              <div className="space-y-2">
                <select
                  value={selectedAccountId ?? ''}
                  onChange={(e) => setSelectedAccountId(Number(e.target.value))}
                  className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
                >
                  {accounts.length === 0 ? (
                    <option value="">No accounts available</option>
                  ) : (
                    accounts.map((account) => (
                      <option key={account.id} value={account.id}>
                        {account.name} ({account.accountType.replace('_', ' ')})
                      </option>
                    ))
                  )}
                </select>
                <button
                  type="button"
                  onClick={() => setShowNewAccountForm(true)}
                  className="flex items-center gap-2 text-sm text-primary font-medium hover:underline"
                >
                  <Plus className="w-4 h-4" />
                  Create new account
                </button>
              </div>
            ) : (
              <div className="border border-slate-200 dark:border-slate-700 rounded-lg p-4 space-y-4 bg-slate-50 dark:bg-slate-800/50">
                <div className="flex items-center gap-2 mb-2">
                  <Wallet className="w-5 h-5 text-primary" />
                  <span className="font-semibold text-sm">Create New Account</span>
                </div>

                <div>
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                    Account Name *
                  </label>
                  <input
                    type="text"
                    value={newAccountName}
                    onChange={(e) => setNewAccountName(e.target.value)}
                    placeholder="e.g., HDFC Savings, SBI Credit Card"
                    className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white placeholder-slate-400"
                  />
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                      Account Type
                    </label>
                    <select
                      value={newAccountType}
                      onChange={(e) => setNewAccountType(e.target.value)}
                      className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white"
                    >
                      {ACCOUNT_TYPES.map((type) => (
                        <option key={type.value} value={type.value}>
                          {type.label}
                        </option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                      Bank / Institution
                    </label>
                    <input
                      type="text"
                      value={newAccountInstitution}
                      onChange={(e) => setNewAccountInstitution(e.target.value)}
                      placeholder="e.g., HDFC Bank"
                      className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 text-slate-900 dark:text-white placeholder-slate-400"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                    Color
                  </label>
                  <div className="flex gap-2">
                    {ACCOUNT_COLORS.map((color) => (
                      <button
                        key={color}
                        type="button"
                        onClick={() => setNewAccountColor(color)}
                        className={cn(
                          'w-8 h-8 rounded-full transition-all',
                          newAccountColor === color && 'ring-2 ring-offset-2 ring-slate-400'
                        )}
                        style={{ backgroundColor: color }}
                      />
                    ))}
                  </div>
                </div>

                <div className="flex gap-2 pt-2">
                  <button
                    type="button"
                    onClick={() => {
                      setShowNewAccountForm(false)
                      setNewAccountName('')
                      setNewAccountType('checking')
                      setNewAccountInstitution('')
                    }}
                    disabled={accounts.length === 0}
                    className="flex-1 py-2 border border-slate-200 dark:border-slate-700 rounded-lg font-medium text-sm hover:bg-white dark:hover:bg-slate-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Cancel
                  </button>
                  <button
                    type="button"
                    onClick={handleCreateAccount}
                    disabled={isCreatingAccount || !newAccountName.trim()}
                    className="flex-1 py-2 bg-primary text-white rounded-lg font-medium text-sm hover:bg-primary/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                  >
                    {isCreatingAccount ? (
                      <>
                        <Loader2 className="w-4 h-4 animate-spin" />
                        Creating...
                      </>
                    ) : (
                      <>
                        <Plus className="w-4 h-4" />
                        Create Account
                      </>
                    )}
                  </button>
                </div>
              </div>
            )}
          </div>

          <TransactionPreviewTable transactions={uploadResult.preview} />

          <div className="flex gap-3">
            <button
              onClick={handleCancel}
              className="flex-1 py-3 border border-slate-200 dark:border-slate-700 rounded-lg font-semibold hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleConfirmImport}
              disabled={isConfirming || !selectedAccountId || showNewAccountForm}
              className="flex-1 py-3 bg-emerald-600 text-white rounded-lg font-semibold hover:bg-emerald-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isConfirming ? (
                <>
                  <Loader2 className="w-5 h-5 animate-spin" />
                  Importing...
                </>
              ) : (
                <>
                  <Check className="w-5 h-5" />
                  Confirm Import
                </>
              )}
            </button>
          </div>
        </div>
      )}
    </div>
  )
}

function TransactionPreviewTable({ transactions }: { transactions: TransactionPreview[] }) {
  const displayTransactions = transactions.slice(0, 10)
  const hasMore = transactions.length > 10

  return (
    <div className="border border-slate-200 dark:border-slate-700 rounded-lg overflow-hidden">
      <div className="px-4 py-3 bg-slate-50 dark:bg-slate-800/50 border-b border-slate-200 dark:border-slate-700">
        <h4 className="font-semibold text-sm">Transaction Preview</h4>
      </div>
      <div className="max-h-64 overflow-y-auto">
        <table className="w-full text-sm">
          <thead className="bg-slate-50 dark:bg-slate-800/30 sticky top-0">
            <tr>
              <th className="px-4 py-2 text-left text-xs font-semibold text-slate-500 uppercase">
                Date
              </th>
              <th className="px-4 py-2 text-left text-xs font-semibold text-slate-500 uppercase">
                Description
              </th>
              <th className="px-4 py-2 text-right text-xs font-semibold text-slate-500 uppercase">
                Amount
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
            {displayTransactions.map((tx, idx) => (
              <tr key={idx} className="hover:bg-slate-50 dark:hover:bg-slate-800/50">
                <td className="px-4 py-2 text-slate-600 dark:text-slate-400">{tx.date}</td>
                <td className="px-4 py-2 truncate max-w-xs" title={tx.description}>
                  {tx.description}
                </td>
                <td
                  className={cn(
                    'px-4 py-2 text-right font-mono',
                    tx.transaction_type === 'credit'
                      ? 'text-emerald-600'
                      : 'text-slate-900 dark:text-white'
                  )}
                >
                  {tx.transaction_type === 'credit' ? '+' : '-'}
                  {tx.amount}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {hasMore && (
        <div className="px-4 py-2 bg-slate-50 dark:bg-slate-800/50 border-t border-slate-200 dark:border-slate-700 text-center text-sm text-slate-500">
          ... and {transactions.length - 10} more transactions
        </div>
      )}
    </div>
  )
}

function StatementsList() {
  const [statements, setStatements] = useState<Statement[]>([])
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    loadStatements()
  }, [])

  const loadStatements = async () => {
    try {
      const data = await statementsApi.list()
      setStatements(data)
    } catch (err) {
      console.error('Failed to load statements:', err)
    } finally {
      setIsLoading(false)
    }
  }

  const handleDelete = async (pid: string) => {
    if (!confirm('Are you sure you want to delete this statement?')) return

    try {
      await statementsApi.delete(pid)
      setStatements((prev) => prev.filter((s) => s.pid !== pid))
    } catch (err) {
      console.error('Failed to delete statement:', err)
    }
  }

  if (isLoading) {
    return (
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm">
        <div className="flex items-center justify-center gap-2 text-slate-500">
          <Loader2 className="w-5 h-5 animate-spin" />
          Loading statements...
        </div>
      </div>
    )
  }

  if (statements.length === 0) {
    return (
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm text-center">
        <FileText className="w-12 h-12 text-slate-300 mx-auto mb-3" />
        <p className="text-slate-500">No statements uploaded yet</p>
      </div>
    )
  }

  return (
    <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl shadow-sm overflow-hidden">
      <div className="px-6 py-4 border-b border-slate-200 dark:border-slate-800">
        <h3 className="text-lg font-bold">Uploaded Statements</h3>
      </div>
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="bg-slate-50 dark:bg-slate-800/30">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                Filename
              </th>
              <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                Type
              </th>
              <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                Status
              </th>
              <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                Transactions
              </th>
              <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                Uploaded
              </th>
              <th className="px-6 py-3 text-right text-xs font-semibold text-slate-500 uppercase">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
            {statements.map((stmt) => (
              <tr key={stmt.pid} className="hover:bg-slate-50 dark:hover:bg-slate-800/50">
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <FileText className="w-5 h-5 text-slate-400" />
                    <span className="font-medium truncate max-w-xs">{stmt.filename}</span>
                  </div>
                </td>
                <td className="px-6 py-4 text-sm text-slate-500 uppercase">{stmt.fileType}</td>
                <td className="px-6 py-4">
                  <StatusBadge status={stmt.status} />
                </td>
                <td className="px-6 py-4 text-sm">{stmt.transactionCount}</td>
                <td className="px-6 py-4 text-sm text-slate-500">
                  {new Date(stmt.createdAt).toLocaleDateString()}
                </td>
                <td className="px-6 py-4 text-right">
                  <button
                    onClick={() => handleDelete(stmt.pid)}
                    className="p-2 text-slate-400 hover:text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-900/20 rounded-lg transition-colors"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}

function StatusBadge({ status }: { status: string }) {
  const styles = {
    pending: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400',
    processing: 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400',
    completed: 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-400',
    failed: 'bg-rose-100 text-rose-800 dark:bg-rose-900/30 dark:text-rose-400',
  }

  return (
    <span
      className={cn(
        'px-2 py-1 rounded-full text-xs font-semibold',
        styles[status as keyof typeof styles] || styles.pending
      )}
    >
      {status}
    </span>
  )
}
