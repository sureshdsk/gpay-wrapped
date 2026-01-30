import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { api } from '@/lib/api/client'
import { authApi } from '@/lib/api/auth'
import type { User } from '@/types'

interface AuthState {
  user: User | null
  token: string | null
  isAuthenticated: boolean
  isLoading: boolean

  setUser: (user: User | null) => void
  setToken: (token: string | null) => void
  login: (email: string, password: string) => Promise<void>
  register: (email: string, password: string, name: string) => Promise<void>
  logout: () => void
  fetchCurrentUser: () => Promise<void>
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      token: null,
      isAuthenticated: false,
      isLoading: false,

      setUser: (user) => set({ user, isAuthenticated: !!user }),

      setToken: (token) => {
        api.setToken(token)
        set({ token, isAuthenticated: !!token })
      },

      login: async (email, password) => {
        set({ isLoading: true })
        try {
          const response = await authApi.login({ email, password })
          api.setToken(response.token)
          set({
            token: response.token,
            user: {
              id: 0, // Not returned from login
              pid: response.pid,
              email: email, // Use the email from the login request
              name: response.name,
              createdAt: new Date().toISOString(),
            },
            isAuthenticated: true,
            isLoading: false,
          })
        } catch (error) {
          set({ isLoading: false })
          throw error
        }
      },

      register: async (email, password, name) => {
        set({ isLoading: true })
        try {
          await authApi.register({ email, password, name })
          set({ isLoading: false })
        } catch (error) {
          set({ isLoading: false })
          throw error
        }
      },

      logout: () => {
        api.setToken(null)
        set({
          user: null,
          token: null,
          isAuthenticated: false,
        })
      },

      fetchCurrentUser: async () => {
        const { token } = get()
        if (!token) return

        set({ isLoading: true })
        try {
          const user = await authApi.getCurrentUser()
          set({ user, isAuthenticated: true, isLoading: false })
        } catch {
          set({
            user: null,
            token: null,
            isAuthenticated: false,
            isLoading: false,
          })
        }
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        token: state.token,
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
      onRehydrateStorage: () => (state) => {
        if (state?.token) {
          api.setToken(state.token)
          state.isAuthenticated = true
        }
      },
    }
  )
)
