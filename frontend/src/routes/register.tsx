import { createFileRoute, Navigate } from '@tanstack/react-router'
import { useAuthStore } from '@/stores/auth-store'
import { RegisterPage } from '@/components/auth/register-page'

export const Route = createFileRoute('/register')({
  component: RegisterRoute,
})

function RegisterRoute() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated)

  if (isAuthenticated) {
    return <Navigate to="/" />
  }

  return <RegisterPage />
}
