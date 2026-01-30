import { Outlet } from '@tanstack/react-router'
import { Sidebar } from './sidebar'
import { Header } from './header'

interface RootLayoutProps {
  title?: string
}

export function RootLayout({ title }: RootLayoutProps) {
  return (
    <div className="flex min-h-screen bg-white dark:bg-slate-950">
      <Sidebar />
      <main className="flex-1 ml-64 min-h-screen">
        <Header breadcrumbs={[{ label: title || 'Dashboard' }]} />
        <div className="p-8">
          <Outlet />
        </div>
      </main>
    </div>
  )
}
