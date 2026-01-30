import { Link, useMatchRoute } from '@tanstack/react-router'
import {
  LayoutDashboard,
  Wallet,
  ArrowLeftRight,
  FileText,
  Settings,
  HelpCircle,
  Sparkles,
} from 'lucide-react'
import { cn } from '@/lib/utils'
import { useAuthStore } from '@/stores/auth-store'

const mainNavItems = [
  { href: '/', label: 'Dashboard', icon: LayoutDashboard },
  { href: '/accounts', label: 'Accounts', icon: Wallet },
  { href: '/transactions', label: 'Transactions', icon: ArrowLeftRight },
  { href: '/statements', label: 'Statements', icon: FileText },
]

const systemNavItems = [
  { href: '/settings/profile', label: 'Settings', icon: Settings },
  { href: '/settings/features', label: 'Features', icon: Sparkles },
]

export function Sidebar() {
  const matchRoute = useMatchRoute()
  const { user } = useAuthStore()

  const isActive = (path: string) => {
    if (path === '/') {
      return matchRoute({ to: '/' })
    }
    return matchRoute({ to: path, fuzzy: true })
  }

  return (
    <aside className="w-64 border-r border-slate-200 dark:border-slate-800 flex flex-col fixed h-full bg-white dark:bg-slate-950 z-20">
      {/* Logo */}
      <div className="p-6 flex items-center gap-3">
        <div className="w-8 h-8 bg-primary rounded-lg flex items-center justify-center text-white">
          <Sparkles className="w-5 h-5" />
        </div>
        <h1 className="text-xl font-black tracking-tight text-slate-900 dark:text-white">
          finn-lens
        </h1>
      </div>

      {/* Navigation */}
      <nav className="flex-1 mt-4">
        <div className="px-3 mb-2">
          <p className="text-[10px] font-bold uppercase tracking-widest text-slate-500 px-3 mb-2">
            Main Menu
          </p>
          {mainNavItems.map((item) => {
            const active = isActive(item.href)
            return (
              <Link
                key={item.href}
                to={item.href}
                className={cn(
                  'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all mt-1',
                  active
                    ? 'border-l-[3px] border-primary bg-primary/10 text-primary'
                    : 'text-slate-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white'
                )}
              >
                <item.icon className="w-5 h-5" />
                <span className="text-sm font-semibold">{item.label}</span>
              </Link>
            )
          })}
        </div>

        <div className="px-3 mt-8">
          <p className="text-[10px] font-bold uppercase tracking-widest text-slate-500 px-3 mb-2">
            System
          </p>
          {systemNavItems.map((item) => {
            const active = isActive(item.href)
            return (
              <Link
                key={item.href}
                to={item.href}
                className={cn(
                  'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-all mt-1',
                  active
                    ? 'border-l-[3px] border-primary bg-primary/10 text-primary'
                    : 'text-slate-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white'
                )}
              >
                <item.icon className="w-5 h-5" />
                <span className="text-sm font-semibold">{item.label}</span>
              </Link>
            )
          })}
          <a
            href="#"
            className="flex items-center gap-3 px-3 py-2.5 rounded-lg text-slate-500 hover:bg-slate-50 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-white transition-all mt-1"
          >
            <HelpCircle className="w-5 h-5" />
            <span className="text-sm font-semibold">Support</span>
          </a>
        </div>
      </nav>

      {/* User Profile */}
      <div className="p-4 border-t border-slate-200 dark:border-slate-800">
        <div className="bg-slate-50 dark:bg-slate-800/50 p-3 rounded-xl flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary font-bold">
            {user?.name?.charAt(0).toUpperCase() || 'U'}
          </div>
          <div className="overflow-hidden">
            <p className="text-sm font-bold truncate">{user?.name || 'User'}</p>
            <p className="text-xs text-slate-500 truncate">Free Plan</p>
          </div>
        </div>
      </div>
    </aside>
  )
}
