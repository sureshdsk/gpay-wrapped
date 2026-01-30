import { Search, Bell, Plus, Moon, Sun } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { useUIStore } from '@/stores/ui-store'

interface HeaderProps {
  breadcrumbs?: { label: string; href?: string }[]
}

export function Header({ breadcrumbs = [{ label: 'Dashboard' }] }: HeaderProps) {
  const { theme, setTheme } = useUIStore()

  const toggleTheme = () => {
    setTheme(theme === 'dark' ? 'light' : 'dark')
  }

  return (
    <header className="h-16 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between px-8 bg-white dark:bg-slate-950 sticky top-0 z-10">
      {/* Breadcrumbs */}
      <div className="flex items-center gap-2">
        <span className="text-slate-500 text-sm">Pages</span>
        {breadcrumbs.map((crumb, index) => (
          <span key={crumb.label} className="flex items-center gap-2">
            <span className="text-slate-500 text-sm">/</span>
            <span
              className={
                index === breadcrumbs.length - 1
                  ? 'text-slate-900 dark:text-white text-sm font-semibold'
                  : 'text-slate-500 text-sm'
              }
            >
              {crumb.label}
            </span>
          </span>
        ))}
      </div>

      {/* Actions */}
      <div className="flex items-center gap-4">
        <button className="p-2 text-slate-500 hover:text-primary transition-colors">
          <Search className="w-5 h-5" />
        </button>
        <button className="p-2 text-slate-500 hover:text-primary transition-colors relative">
          <Bell className="w-5 h-5" />
          <span className="absolute top-2 right-2 w-2 h-2 bg-red-500 border-2 border-white dark:border-slate-950 rounded-full" />
        </button>
        <button
          onClick={toggleTheme}
          className="p-2 text-slate-500 hover:text-primary transition-colors"
        >
          {theme === 'dark' ? (
            <Sun className="w-5 h-5" />
          ) : (
            <Moon className="w-5 h-5" />
          )}
        </button>
        <div className="h-6 w-[1px] bg-slate-200 dark:bg-slate-800 mx-2" />
        <Button size="sm">
          <Plus className="w-4 h-4" />
          New Transaction
        </Button>
      </div>
    </header>
  )
}
