import { api } from './client'
import type { AuthResponse, User } from '@/types'

interface RegisterParams {
  email: string
  password: string
  name: string
}

interface LoginParams {
  email: string
  password: string
}

export const authApi = {
  register: (params: RegisterParams) =>
    api.post<void>('/auth/register', params),

  login: (params: LoginParams) =>
    api.post<AuthResponse>('/auth/login', params),

  logout: () => api.post<void>('/auth/logout'),

  getCurrentUser: () => api.get<User>('/auth/current'),

  forgotPassword: (email: string) =>
    api.post<void>('/auth/forgot', { email }),

  resetPassword: (token: string, password: string) =>
    api.post<void>('/auth/reset', { token, password }),

  verifyEmail: (token: string) =>
    api.get<void>(`/auth/verify/${token}`),
}
