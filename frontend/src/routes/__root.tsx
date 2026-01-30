import { createRootRoute, Outlet, useLocation } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { useAuthStore } from '@/stores/auth-store'
import { Sidebar } from '@/components/layout/sidebar'
import { Header } from '@/components/layout/header'

export const Route = createRootRoute({
  component: RootComponent,
})

const publicRoutes = ['/login', '/register']

function RootComponent() {
  const location = useLocation()
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)
  const isPublicRoute = publicRoutes.includes(location.pathname)

  // Show layout with sidebar for authenticated users on non-public routes
  if (isAuthenticated && !isPublicRoute) {
    return (
      <>
        <div className="flex min-h-screen bg-white dark:bg-slate-950">
          <Sidebar />
          <main className="flex-1 ml-64 min-h-screen">
            <Header breadcrumbs={[{ label: getPageTitle(location.pathname) }]} />
            <div className="p-8">
              <Outlet />
            </div>
          </main>
        </div>
        {import.meta.env.DEV && <TanStackRouterDevtools />}
      </>
    )
  }

  // Show plain outlet for public routes (login, register)
  return (
    <>
      <Outlet />
      {import.meta.env.DEV && <TanStackRouterDevtools />}
    </>
  )
}

function getPageTitle(pathname: string): string {
  const titles: Record<string, string> = {
    '/': 'Dashboard',
    '/accounts': 'Accounts',
    '/transactions': 'Transactions',
    '/statements': 'Statements',
    '/settings/profile': 'Settings',
    '/settings/features': 'Features',
  }
  return titles[pathname] || 'Dashboard'
}
