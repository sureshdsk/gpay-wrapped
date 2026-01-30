import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { useState, useEffect } from 'react'
import { accountsApi } from '@/lib/api/accounts'
import type { Account } from '@/types'
import {
  Wallet,
  Plus,
  Loader2,
  Pencil,
  Trash2,
  CreditCard,
  PiggyBank,
  TrendingUp,
  Building2,
  X,
  Check
} from 'lucide-react'
import { cn } from '@/lib/utils'

export const Route = createFileRoute('/accounts')({
  component: AccountsPage,
})

const ACCOUNT_TYPES = [
  { value: 'checking', label: 'Checking', icon: Wallet },
  { value: 'savings', label: 'Savings', icon: PiggyBank },
  { value: 'credit_card', label: 'Credit Card', icon: CreditCard },
  { value: 'investment', label: 'Investment', icon: TrendingUp },
]

const ACCOUNT_COLORS = [
  '#3B82F6', '#10B981', '#F59E0B', '#EF4444',
  '#8B5CF6', '#EC4899', '#06B6D4', '#84CC16',
]

function AccountsPage() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)
  const [accounts, setAccounts] = useState<Account[]>([])
  const [totalBalance, setTotalBalance] = useState('0')
  const [isLoading, setIsLoading] = useState(true)
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [editingAccount, setEditingAccount] = useState<Account | null>(null)

  if (!isAuthenticated) {
    return <Navigate to="/login" />
  }

  useEffect(() => {
    loadAccounts()
  }, [])

  const loadAccounts = async () => {
    try {
      const data = await accountsApi.list()
      setAccounts(data.accounts)
      setTotalBalance(data.totalBalance)
    } catch (err) {
      console.error('Failed to load accounts:', err)
    } finally {
      setIsLoading(false)
    }
  }

  const handleDelete = async (pid: string) => {
    if (!confirm('Are you sure you want to delete this account?')) return
    try {
      await accountsApi.delete(pid)
      setAccounts(prev => prev.filter(a => a.pid !== pid))
    } catch (err) {
      console.error('Failed to delete account:', err)
    }
  }

  const handleAccountSaved = (account: Account, isNew: boolean) => {
    if (isNew) {
      setAccounts(prev => [...prev, account])
    } else {
      setAccounts(prev => prev.map(a => a.pid === account.pid ? account : a))
    }
    setShowCreateModal(false)
    setEditingAccount(null)
    loadAccounts() // Refresh to get updated totals
  }

  const getAccountIcon = (type: string) => {
    const accountType = ACCOUNT_TYPES.find(t => t.value === type)
    return accountType?.icon || Wallet
  }

  const formatBalance = (balance: string) => {
    const num = parseFloat(balance)
    return new Intl.NumberFormat('en-IN', {
      style: 'currency',
      currency: 'INR',
      minimumFractionDigits: 2,
    }).format(num)
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
            Accounts
          </h2>
          <p className="text-slate-500 mt-1">Manage your bank accounts and track balances</p>
        </div>
        <button
          onClick={() => setShowCreateModal(true)}
          className="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg font-semibold hover:bg-primary/90 transition-colors"
        >
          <Plus className="w-5 h-5" />
          Add Account
        </button>
      </div>

      {/* Total Balance Card */}
      <div className="bg-gradient-to-r from-primary to-blue-600 rounded-2xl p-6 text-white">
        <p className="text-blue-100 text-sm font-medium">Total Balance</p>
        <p className="text-4xl font-black mt-1">{formatBalance(totalBalance)}</p>
        <p className="text-blue-100 text-sm mt-2">{accounts.length} account{accounts.length !== 1 ? 's' : ''}</p>
      </div>

      {/* Accounts List */}
      {isLoading ? (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm">
          <div className="flex items-center justify-center gap-2 text-slate-500">
            <Loader2 className="w-5 h-5 animate-spin" />
            Loading accounts...
          </div>
        </div>
      ) : accounts.length === 0 ? (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-8 shadow-sm text-center">
          <Wallet className="w-12 h-12 text-slate-300 mx-auto mb-3" />
          <p className="text-slate-500 mb-4">No accounts yet</p>
          <button
            onClick={() => setShowCreateModal(true)}
            className="inline-flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg font-semibold hover:bg-primary/90 transition-colors"
          >
            <Plus className="w-5 h-5" />
            Add Your First Account
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {accounts.map(account => {
            const Icon = getAccountIcon(account.accountType)
            return (
              <div
                key={account.pid}
                className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-5 shadow-sm hover:shadow-md transition-shadow"
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <div
                      className="w-10 h-10 rounded-lg flex items-center justify-center"
                      style={{ backgroundColor: account.color + '20', color: account.color }}
                    >
                      <Icon className="w-5 h-5" />
                    </div>
                    <div>
                      <h3 className="font-semibold text-slate-900 dark:text-white">{account.name}</h3>
                      <p className="text-xs text-slate-500 capitalize">
                        {account.accountType.replace('_', ' ')}
                      </p>
                    </div>
                  </div>
                  <div className="flex gap-1">
                    <button
                      onClick={() => setEditingAccount(account)}
                      className="p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
                    >
                      <Pencil className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleDelete(account.pid)}
                      className="p-2 text-slate-400 hover:text-rose-600 hover:bg-rose-50 dark:hover:bg-rose-900/20 rounded-lg transition-colors"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>

                {account.institution && (
                  <div className="flex items-center gap-2 text-sm text-slate-500 mb-3">
                    <Building2 className="w-4 h-4" />
                    {account.institution}
                  </div>
                )}

                <div className="pt-3 border-t border-slate-100 dark:border-slate-800">
                  <p className="text-xs text-slate-500 mb-1">Current Balance</p>
                  <p className={cn(
                    "text-xl font-bold",
                    parseFloat(account.currentBalance) >= 0
                      ? "text-slate-900 dark:text-white"
                      : "text-rose-600"
                  )}>
                    {formatBalance(account.currentBalance)}
                  </p>
                </div>
              </div>
            )
          })}
        </div>
      )}

      {/* Create/Edit Modal */}
      {(showCreateModal || editingAccount) && (
        <AccountModal
          account={editingAccount}
          onClose={() => {
            setShowCreateModal(false)
            setEditingAccount(null)
          }}
          onSave={handleAccountSaved}
        />
      )}
    </div>
  )
}

interface AccountModalProps {
  account: Account | null
  onClose: () => void
  onSave: (account: Account, isNew: boolean) => void
}

function AccountModal({ account, onClose, onSave }: AccountModalProps) {
  const isEditing = !!account
  const [name, setName] = useState(account?.name || '')
  const [accountType, setAccountType] = useState(account?.accountType || 'checking')
  const [institution, setInstitution] = useState(account?.institution || '')
  const [currentBalance, setCurrentBalance] = useState(account?.currentBalance || '0')
  const [color, setColor] = useState(account?.color || ACCOUNT_COLORS[0])
  const [isSubmitting, setIsSubmitting] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!name.trim()) {
      setError('Account name is required')
      return
    }

    setIsSubmitting(true)
    setError(null)

    try {
      if (isEditing && account) {
        const updated = await accountsApi.update(account.pid, {
          name: name.trim(),
          institution: institution.trim() || undefined,
          color,
          currentBalance,
        })
        onSave(updated, false)
      } else {
        const created = await accountsApi.create({
          name: name.trim(),
          accountType,
          institution: institution.trim() || undefined,
          currentBalance,
          color,
        })
        onSave(created, true)
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save account')
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-white dark:bg-slate-900 rounded-2xl w-full max-w-md shadow-xl">
        <div className="flex items-center justify-between p-6 border-b border-slate-200 dark:border-slate-800">
          <h3 className="text-lg font-bold">
            {isEditing ? 'Edit Account' : 'Add New Account'}
          </h3>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-4">
          {error && (
            <div className="p-3 bg-rose-50 dark:bg-rose-900/20 border border-rose-200 dark:border-rose-800 rounded-lg text-sm text-rose-600">
              {error}
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
              Account Name *
            </label>
            <input
              type="text"
              value={name}
              onChange={e => setName(e.target.value)}
              placeholder="e.g., HDFC Savings"
              className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
            />
          </div>

          {!isEditing && (
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                Account Type
              </label>
              <select
                value={accountType}
                onChange={e => setAccountType(e.target.value as Account['accountType'])}
                className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
              >
                {ACCOUNT_TYPES.map(type => (
                  <option key={type.value} value={type.value}>
                    {type.label}
                  </option>
                ))}
              </select>
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
              Bank / Institution
            </label>
            <input
              type="text"
              value={institution}
              onChange={e => setInstitution(e.target.value)}
              placeholder="e.g., HDFC Bank"
              className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
              Current Balance
            </label>
            <input
              type="number"
              step="0.01"
              value={currentBalance}
              onChange={e => setCurrentBalance(e.target.value)}
              className="w-full px-3 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
              Color
            </label>
            <div className="flex gap-2">
              {ACCOUNT_COLORS.map(c => (
                <button
                  key={c}
                  type="button"
                  onClick={() => setColor(c)}
                  className={cn(
                    'w-8 h-8 rounded-full transition-all',
                    color === c && 'ring-2 ring-offset-2 ring-slate-400'
                  )}
                  style={{ backgroundColor: c }}
                />
              ))}
            </div>
          </div>

          <div className="flex gap-3 pt-4">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 py-2.5 border border-slate-200 dark:border-slate-700 rounded-lg font-semibold hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={isSubmitting}
              className="flex-1 py-2.5 bg-primary text-white rounded-lg font-semibold hover:bg-primary/90 transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
            >
              {isSubmitting ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  Saving...
                </>
              ) : (
                <>
                  <Check className="w-4 h-4" />
                  {isEditing ? 'Save Changes' : 'Create Account'}
                </>
              )}
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}
