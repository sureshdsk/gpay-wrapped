import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { insightsApi } from '@/lib/api'
import { useQuery } from '@tanstack/react-query'

export const Route = createFileRoute('/')({
  component: IndexComponent,
})

function IndexComponent() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)

  if (!isAuthenticated) {
    return <Navigate to="/login" />
  }

  const { data: dashboard, isLoading } = useQuery({
    queryKey: ['dashboard-summary'],
    queryFn: () => insightsApi.getDashboardSummary(),
  })

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="material-symbols-outlined animate-spin text-4xl text-primary">
          refresh
        </div>
      </div>
    )
  }

  const formatCurrency = (value: string) => {
    const num = parseFloat(value)
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(num)
  }

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })
  }

  const totalBalance = dashboard?.totalBalance ?? "0"
  const income = dashboard?.thisMonth.income ?? "0"
  const expenses = dashboard?.thisMonth.expenses ?? "0"
  const accountCount = dashboard?.accountCount ?? 0
  const net = dashboard?.thisMonth.net ?? "0"
  const recentTransactions = dashboard?.recentTransactions ?? []

  const netNum = parseFloat(net)

  return (
    <div className="space-y-6">
      {/* Welcome Section */}
      <div>
        <h2 className="text-3xl font-black text-slate-900 dark:text-white tracking-tight">
          Welcome back
        </h2>
        <p className="text-slate-500 mt-1">Here is what is happening with your money today.</p>
      </div>

      {/* Summary Card Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <SummaryCard
          title="Total Balance"
          value={formatCurrency(totalBalance)}
          icon="account_balance"
          color="blue"
        />
        <SummaryCard
          title="Monthly Income"
          value={formatCurrency(income)}
          icon="payments"
          color="emerald"
        />
        <SummaryCard
          title="Monthly Expenses"
          value={formatCurrency(expenses)}
          icon="shopping_cart"
          color="rose"
        />
        <SummaryCard
          title="Active Accounts"
          value={accountCount.toString()}
          icon="credit_card"
          color="slate"
        />
      </div>

      {/* Monthly Summary */}
      <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-6 shadow-sm">
        <h3 className="text-lg font-bold mb-4">{dashboard?.thisMonth.month ?? 'This Month'}</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div>
            <p className="text-sm text-slate-500 mb-1">Income</p>
            <p className="font-mono text-xl font-bold text-emerald-600">
              {formatCurrency(income)}
            </p>
          </div>
          <div>
            <p className="text-sm text-slate-500 mb-1">Expenses</p>
            <p className="font-mono text-xl font-bold text-rose-600">
              {formatCurrency(expenses)}
            </p>
          </div>
          <div>
            <p className="text-sm text-slate-500 mb-1">Net</p>
            <p className={`font-mono text-xl font-bold ${netNum >= 0 ? 'text-emerald-600' : 'text-rose-600'}`}>
              {formatCurrency(net)}
            </p>
          </div>
        </div>
      </div>

      {/* Recent Transactions */}
      {recentTransactions.length > 0 && (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl overflow-hidden shadow-sm">
          <div className="px-6 py-5 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
            <h3 className="text-lg font-bold">Recent Transactions</h3>
            <button className="text-primary text-sm font-bold hover:underline">
              View All
            </button>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full text-left">
              <thead className="bg-slate-50 dark:bg-slate-800/30">
                <tr>
                  <th className="px-6 py-3 text-[11px] font-bold uppercase tracking-widest text-slate-500">
                    Date
                  </th>
                  <th className="px-6 py-3 text-[11px] font-bold uppercase tracking-widest text-slate-500">
                    Description
                  </th>
                  <th className="px-6 py-3 text-[11px] font-bold uppercase tracking-widest text-slate-500">
                    Category
                  </th>
                  <th className="px-6 py-3 text-[11px] font-bold uppercase tracking-widest text-slate-500 text-right">
                    Amount
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100 dark:divide-slate-800">
                {recentTransactions.map((txn) => (
                  <TransactionRow
                    key={txn.id}
                    date={formatDate(txn.transactionDate)}
                    description={txn.description}
                    category={txn.categoryName ?? 'Uncategorized'}
                    amount={txn.amount}
                    type={txn.transactionType}
                  />
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Empty State - No Transactions */}
      {recentTransactions.length === 0 && (
        <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-xl p-12 shadow-sm text-center">
          <div className="w-16 h-16 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-400 mx-auto mb-4">
            <span className="material-symbols-outlined text-[32px]">
              receipt_long
            </span>
          </div>
          <h4 className="text-lg font-bold text-slate-900 dark:text-white mb-2">No transactions yet</h4>
          <p className="text-slate-500 max-w-sm mx-auto">
            Upload a bank statement to start tracking your finances
          </p>
        </div>
      )}
    </div>
  )
}

interface SummaryCardProps {
  title: string
  value: string
  icon: string
  color: 'blue' | 'emerald' | 'rose' | 'slate'
}

function SummaryCard({
  title,
  value,
  icon,
  color,
}: SummaryCardProps) {
  const colorClasses = {
    blue: 'bg-blue-50 dark:bg-blue-900/30 text-primary',
    emerald: 'bg-emerald-50 dark:bg-emerald-900/30 text-emerald-600',
    rose: 'bg-rose-50 dark:bg-rose-900/30 text-rose-600',
    slate: 'bg-slate-100 dark:bg-slate-800 text-slate-900 dark:text-white',
  }

  const valueColorClass = color === 'emerald' ? 'text-emerald-600' : color === 'rose' ? 'text-rose-600' : 'text-slate-900 dark:text-white'

  return (
    <div className="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow">
      <div className="flex items-center justify-between mb-4">
        <span className="text-slate-500 font-semibold text-xs uppercase tracking-wider">
          {title}
        </span>
        <div className={`w-8 h-8 rounded-lg ${colorClasses[color]} flex items-center justify-center`}>
          <span className="material-symbols-outlined text-[20px]">{icon}</span>
        </div>
      </div>
      <div className="flex flex-col">
        <span className={`font-mono text-2xl font-bold ${valueColorClass}`}>
          {value}
        </span>
      </div>
    </div>
  )
}

interface TransactionRowProps {
  date: string
  description: string
  category: string
  amount: string
  type?: 'debit' | 'credit'
}

function TransactionRow({
  date,
  description,
  category,
  amount,
  type = 'debit',
}: TransactionRowProps) {
  const formatAmount = (amount: string) => {
    const num = parseFloat(amount)
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(Math.abs(num))
  }

  const getIcon = (description: string) => {
    const desc = description.toLowerCase()
    if (desc.includes('food') || desc.includes('restaurant') || desc.includes('bistro')) return 'restaurant'
    if (desc.includes('store') || desc.includes('shop')) return 'shopping_bag'
    if (desc.includes('uber') || desc.includes('lyft') || desc.includes('transport')) return 'directions_car'
    if (desc.includes('power') || desc.includes('electric') || desc.includes('utility')) return 'electric_bolt'
    if (desc.includes('salary') || desc.includes('payroll') || desc.includes('payout')) return 'payments'
    return 'payment'
  }

  return (
    <tr className="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
      <td className="px-6 py-4 text-sm text-slate-500 font-medium">{date}</td>
      <td className="px-6 py-4">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded bg-slate-100 dark:bg-slate-800 flex items-center justify-center text-slate-900 dark:text-white">
            <span className="material-symbols-outlined text-[18px]">{getIcon(description)}</span>
          </div>
          <div className="flex flex-col">
            <span className="text-sm font-semibold">{description}</span>
          </div>
        </div>
      </td>
      <td className="px-6 py-4">
        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-slate-300"></span>
          <span className="text-sm">{category}</span>
        </div>
      </td>
      <td
        className={`px-6 py-4 text-right font-mono text-sm font-semibold ${
          type === 'credit' ? 'text-emerald-600' : 'text-slate-900 dark:text-white'
        }`}
      >
        {type === 'credit' ? '+' : '-'}${formatAmount(amount)}
      </td>
    </tr>
  )
}
