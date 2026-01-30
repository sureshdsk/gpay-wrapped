import { useState } from 'react'
import { Link, useNavigate } from '@tanstack/react-router'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { ArrowRight, Info } from 'lucide-react'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent } from '@/components/ui/card'
import { Checkbox } from '@/components/ui/checkbox'
import { Logo } from '@/components/shared/logo'
import { useAuthStore } from '@/stores/auth-store'

const registerSchema = z
  .object({
    name: z.string().min(2, 'Name must be at least 2 characters'),
    email: z.string().email('Invalid email address'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string(),
    terms: z.boolean().refine((val) => val === true, {
      message: 'You must accept the terms and conditions',
    }),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords don't match",
    path: ['confirmPassword'],
  })

type RegisterForm = z.infer<typeof registerSchema>

function getPasswordStrength(password: string): {
  level: number
  label: string
} {
  let score = 0
  if (password.length >= 8) score++
  if (password.length >= 12) score++
  if (/[A-Z]/.test(password)) score++
  if (/[a-z]/.test(password)) score++
  if (/[0-9]/.test(password)) score++
  if (/[^A-Za-z0-9]/.test(password)) score++

  if (score <= 2) return { level: 1, label: 'Weak' }
  if (score <= 4) return { level: 2, label: 'Medium' }
  return { level: 3, label: 'Strong' }
}

export function RegisterPage() {
  const navigate = useNavigate()
  const { register: registerUser, isLoading } = useAuthStore()
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<RegisterForm>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      terms: false,
    },
  })

  const password = watch('password', '')
  const strength = getPasswordStrength(password)

  const onSubmit = async (data: RegisterForm) => {
    setError(null)
    try {
      await registerUser(data.email, data.password, data.name)
      setSuccess(true)
      setTimeout(() => {
        navigate({ to: '/login' })
      }, 2000)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Registration failed')
    }
  }

  if (success) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-slate-50 dark:bg-slate-950 p-6">
        <Card className="max-w-md w-full">
          <CardContent className="p-8 text-center">
            <div className="w-16 h-16 mx-auto mb-4 bg-green-100 dark:bg-green-900/20 rounded-full flex items-center justify-center">
              <svg
                className="w-8 h-8 text-green-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
            <h2 className="text-xl font-bold text-slate-900 dark:text-white mb-2">
              Account Created!
            </h2>
            <p className="text-slate-500 dark:text-slate-400 text-sm">
              Please check your email to verify your account. Redirecting to
              login...
            </p>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="min-h-screen flex flex-col bg-slate-50 dark:bg-slate-950">
      {/* Top Navigation */}
      <header className="w-full bg-white/80 dark:bg-slate-950/80 backdrop-blur-md border-b border-slate-200 dark:border-slate-800 sticky top-0 z-50">
        <div className="max-w-[1200px] mx-auto px-6 h-16 flex items-center justify-between">
          <Logo />
          <div className="flex items-center gap-4">
            <span className="text-sm text-slate-500 hidden sm:inline">
              Already have an account?
            </span>
            <Link to="/login">
              <Button variant="outline" size="sm">
                Sign In
              </Button>
            </Link>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-grow flex items-center justify-center p-6 py-12">
        <div className="w-full max-w-[480px]">
          <Card>
            <div className="p-8 pb-4">
              <h1 className="text-2xl font-bold text-slate-900 dark:text-white tracking-tight">
                Create your account
              </h1>
              <p className="text-slate-500 dark:text-slate-400 text-sm mt-2">
                Start your financial journey with precision tracking.
              </p>
            </div>

            {error && (
              <div className="mx-8 mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-sm text-red-600 dark:text-red-400">
                {error}
              </div>
            )}

            <form
              onSubmit={handleSubmit(onSubmit)}
              className="p-8 pt-4 space-y-5"
            >
              {/* Full Name */}
              <div className="space-y-1.5">
                <Label htmlFor="name">Full Name</Label>
                <Input
                  id="name"
                  type="text"
                  placeholder="John Doe"
                  {...register('name')}
                />
                {errors.name && (
                  <p className="text-xs text-red-500">{errors.name.message}</p>
                )}
              </div>

              {/* Email */}
              <div className="space-y-1.5">
                <Label htmlFor="email">Email address</Label>
                <Input
                  id="email"
                  type="email"
                  placeholder="name@company.com"
                  {...register('email')}
                />
                {errors.email && (
                  <p className="text-xs text-red-500">{errors.email.message}</p>
                )}
              </div>

              {/* Password */}
              <div className="space-y-1.5">
                <Label htmlFor="password">Password</Label>
                <Input
                  id="password"
                  type="password"
                  placeholder="••••••••"
                  {...register('password')}
                />
                {/* Strength Indicator */}
                <div className="flex gap-1 mt-2">
                  {[1, 2, 3, 4].map((segment) => (
                    <div
                      key={segment}
                      className={`h-1 flex-1 rounded-full transition-colors ${
                        password &&
                        segment <=
                          (strength.level === 1 ? 1 : strength.level === 2 ? 2 : 4)
                          ? strength.level === 1
                            ? 'bg-red-500'
                            : strength.level === 2
                            ? 'bg-amber-500'
                            : 'bg-green-500'
                          : 'bg-slate-200 dark:bg-slate-700'
                      }`}
                    />
                  ))}
                </div>
                {password && (
                  <p className="text-[11px] text-slate-500 mt-1 flex items-center gap-1">
                    <Info className="w-3 h-3" />
                    Strength: {strength.label}. Mix letters, numbers, and symbols.
                  </p>
                )}
                {errors.password && (
                  <p className="text-xs text-red-500">
                    {errors.password.message}
                  </p>
                )}
              </div>

              {/* Confirm Password */}
              <div className="space-y-1.5">
                <Label htmlFor="confirmPassword">Confirm Password</Label>
                <Input
                  id="confirmPassword"
                  type="password"
                  placeholder="••••••••"
                  {...register('confirmPassword')}
                />
                {errors.confirmPassword && (
                  <p className="text-xs text-red-500">
                    {errors.confirmPassword.message}
                  </p>
                )}
              </div>

              {/* Terms Checkbox */}
              <div className="flex items-start gap-3 py-2">
                <Checkbox id="terms" {...register('terms')} />
                <label
                  htmlFor="terms"
                  className="text-xs text-slate-500 dark:text-slate-400 leading-normal cursor-pointer"
                >
                  I agree to the{' '}
                  <a
                    href="#"
                    className="text-primary hover:underline font-medium"
                  >
                    Terms of Service
                  </a>{' '}
                  and{' '}
                  <a
                    href="#"
                    className="text-primary hover:underline font-medium"
                  >
                    Privacy Policy
                  </a>
                  .
                </label>
              </div>
              {errors.terms && (
                <p className="text-xs text-red-500">{errors.terms.message}</p>
              )}

              {/* Submit Button */}
              <Button
                type="submit"
                className="w-full"
                size="lg"
                disabled={isLoading}
              >
                {isLoading ? (
                  'Creating account...'
                ) : (
                  <>
                    <span>Create Account</span>
                    <ArrowRight className="h-4 w-4" />
                  </>
                )}
              </Button>
            </form>

            {/* Footer Gradient */}
            <div className="h-1 w-full bg-gradient-to-r from-transparent via-primary/30 to-transparent opacity-50" />
          </Card>

          {/* Bottom Support Text */}
          <p className="text-center text-slate-400 text-sm mt-8">
            By joining, you'll receive monthly financial insights via email.
            <br />
            You can opt-out at any time.
          </p>
        </div>
      </main>
    </div>
  )
}
