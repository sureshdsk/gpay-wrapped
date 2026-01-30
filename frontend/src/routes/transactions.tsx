import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { useState, useEffect, useCallback } from 'react'
import { transactionsApi } from '@/lib/api/transactions'
import { accountsApi } from '@/lib/api/accounts'
import type { Transaction, Account, TransactionFilters } from '@/types'
import {
  ArrowDownLeft,
  ArrowUpRight,
  Loader2,
  Search,
  Filter,
  X,
  ChevronLeft,
  ChevronRight,
  Calendar,
} from 'lucide-react'
import { cn } from '@/lib/utils'

export const Route = createFileRoute('/transactions')({
  component: TransactionsPage,
})

function TransactionsPage() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)
  const [transactions, setTransactions] = useState<Transaction[]>([])
  const [accounts, setAccounts] = useState<Account[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [page, setPage] = useState(1)
  const [showFilters, setShowFilters] = useState(false)

  // Filter state
  const [filters] = useState<TransactionFilters>({
    perPage: 20,
  })
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedAccountId, setSelectedAccountId] = useState<number | undefined>()
  const [transactionType, setTransactionType] = useState<'debit' | 'credit' | undefined>()
  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')

  if (!isAuthenticated) {
    return <Navigate to="/login" />
  }

  const loadTransactions = useCallback(async () => {
    setIsLoading(true)
    try {
      const activeFilters: TransactionFilters = {
        ...filters,
        page,
        search: searchTerm || undefined,
        accountId: selectedAccountId,
        transactionType,
        startDate: startDate || undefined,
        endDate: endDate || undefined,
      }
      const data = await transactionsApi.list(activeFilters)
      setTransactions(data.transactions)
    } catch (err) {
      console.error('Failed to load transactions:', err)
    } finally {
      setIsLoading(false)
    }
  }, [filters, page, searchTerm, selectedAccountId, transactionType, startDate, endDate])

  useEffect(() => {
    loadTransactions()
  }, [loadTransactions])

  useEffect(() => {
    accountsApi.list().then(data => setAccounts(data.accounts)).catch(console.error)
  }, [])

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault()
    setPage(1)
    loadTransactions()
  }

  const clearFilters = () => {
    setSearchTerm('')
    setSelectedAccountId(undefined)
    setTransactionType(undefined)
    setStartDate('')
    setEndDate('')
    setPage(1)
  }

  const hasActiveFilters = searchTerm || selectedAccountId || transactionType || startDate || endDate

  const formatAmount = (amount: string, type: 'debit' | 'credit') => {
    const num = parseFloat(amount)
    const formatted = new Intl.NumberFormat('en-IN', {
      style: 'currency',
      currency: 'INR',
      minimumFractionDigits: 2,
    }).format(num)
    return type === 'credit' ? `+${formatted}` : `-${formatted}`
  }

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-IN', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
    })
  }

  const getAccountName = (accountId: number) => {
    const account = accounts.find(a => a.id === accountId)
    return account?.name || 'Unknown'
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
          Transactions
        </h2>
        <p className="text-slate-500 mt-1">View and manage all your transactions</p>
      </div>

      {/* Search and Filters */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-4 shadow-sm">
        <div className="flex flex-col md:flex-row gap-4">
          <form onSubmit={handleSearch} className="flex-1 flex gap-2">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
              <input
                type="text"
                value={searchTerm}
                onChange={e => setSearchTerm(e.target.value)}
                placeholder="Search transactions..."
                className="w-full pl-10 pr-4 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
              />
            </div>
            <button
              type="submit"
              className="px-4 py-2 bg-primary text-white rounded-lg font-medium hover:bg-primary/90 transition-colors"
            >
              Search
            </button>
          </form>

          <button
            onClick={() => setShowFilters(!showFilters)}
            className={cn(
              "flex items-center gap-2 px-4 py-2 border rounded-lg font-medium transition-colors",
              showFilters || hasActiveFilters
                ? "border-primary text-primary bg-primary/5"
                : "border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800"
            )}
          >
            <Filter className="w-5 h-5" />
            Filters
            {hasActiveFilters && (
              <span className="w-2 h-2 rounded-full bg-primary" />
            )}
          </button>
        </div>

        {/* Filter Panel */}
        {showFilters && (
          <div className="mt-4 pt-4 border-t border-slate-200 dark:border-slate-700">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  Account
                </label>
                <select
                  value={selectedAccountId || ''}
                  onChange={e => setSelectedAccountId(e.target.value ? Number(e.target.value) : undefined)}
                  className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
                >
                  <option value="">All Accounts</option>
                  {accounts.map(account => (
                    <option key={account.id} value={account.id}>
                      {account.name}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  Type
                </label>
                <select
                  value={transactionType || ''}
                  onChange={e => setTransactionType(e.target.value as 'debit' | 'credit' | undefined || undefined)}
                  className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
                >
                  <option value="">All Types</option>
                  <option value="debit">Expenses (Debit)</option>
                  <option value="credit">Income (Credit)</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  From Date
                </label>
                <input
                  type="date"
                  value={startDate}
                  onChange={e => setStartDate(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  To Date
                </label>
                <input
                  type="date"
                  value={endDate}
                  onChange={e => setEndDate(e.target.value)}
                  className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
                />
              </div>
            </div>

            {hasActiveFilters && (
              <button
                onClick={clearFilters}
                className="mt-4 flex items-center gap-2 text-sm text-slate-500 hover:text-slate-700"
              >
                <X className="w-4 h-4" />
                Clear all filters
              </button>
            )}
          </div>
        )}
      </div>

      {/* Transactions List */}
      {isLoading ? (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm">
          <div className="flex items-center justify-center gap-2 text-slate-500">
            <Loader2 className="w-5 h-5 animate-spin" />
            Loading transactions...
          </div>
        </div>
      ) : transactions.length === 0 ? (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm text-center">
          <Calendar className="w-12 h-12 text-slate-300 mx-auto mb-3" />
          <p className="text-slate-500">
            {hasActiveFilters ? 'No transactions match your filters' : 'No transactions yet'}
          </p>
          {hasActiveFilters && (
            <button
              onClick={clearFilters}
              className="mt-4 text-primary font-medium hover:underline"
            >
              Clear filters
            </button>
          )}
        </div>
      ) : (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl shadow-sm overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-slate-50 dark:bg-slate-800/30">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                    Date
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                    Description
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-semibold text-slate-500 uppercase">
                    Account
                  </th>
                  <th className="px-6 py-3 text-right text-xs font-semibold text-slate-500 uppercase">
                    Amount
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
                {transactions.map(tx => (
                  <tr key={tx.pid} className="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
                    <td className="px-6 py-4 text-sm text-slate-500">
                      {formatDate(tx.transactionDate)}
                    </td>
                    <td className="px-6 py-4">
                      <div className="flex items-center gap-3">
                        <div className={cn(
                          "w-8 h-8 rounded-lg flex items-center justify-center",
                          tx.transactionType === 'credit'
                            ? "bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30"
                            : "bg-slate-100 text-slate-600 dark:bg-slate-800"
                        )}>
                          {tx.transactionType === 'credit' ? (
                            <ArrowDownLeft className="w-4 h-4" />
                          ) : (
                            <ArrowUpRight className="w-4 h-4" />
                          )}
                        </div>
                        <div>
                          <p className="font-medium text-slate-900 dark:text-white">
                            {tx.description}
                          </p>
                          {tx.merchantName && (
                            <p className="text-xs text-slate-500">{tx.merchantName}</p>
                          )}
                        </div>
                      </div>
                    </td>
                    <td className="px-6 py-4 text-sm text-slate-500">
                      {getAccountName(tx.accountId)}
                    </td>
                    <td className={cn(
                      "px-6 py-4 text-right font-mono font-semibold",
                      tx.transactionType === 'credit'
                        ? "text-emerald-600"
                        : "text-slate-900 dark:text-white"
                    )}>
                      {formatAmount(tx.amount, tx.transactionType)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          {/* Pagination */}
          <div className="px-6 py-4 border-t border-slate-200 dark:border-slate-800 flex items-center justify-between">
            <p className="text-sm text-slate-500">
              Page {page}
            </p>
            <div className="flex gap-2">
              <button
                onClick={() => setPage(p => Math.max(1, p - 1))}
                disabled={page === 1}
                className="p-2 border border-slate-200 dark:border-slate-700 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <ChevronLeft className="w-5 h-5" />
              </button>
              <button
                onClick={() => setPage(p => p + 1)}
                disabled={transactions.length < (filters.perPage || 20)}
                className="p-2 border border-slate-200 dark:border-slate-700 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <ChevronRight className="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
