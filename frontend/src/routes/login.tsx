import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { LoginPage } from '@/components/auth/login-page'

export const Route = createFileRoute('/login')({
  component: LoginRoute,
})

function LoginRoute() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)

  if (isAuthenticated) {
    return <Navigate to="/" />
  }

  return <LoginPage />
}
