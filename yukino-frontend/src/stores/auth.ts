import { defineStore } from 'pinia'

const TOKEN_KEY = 'yukino.token'
const USER_KEY = 'yukino.user'

export type UserRole = 'user' | 'admin'

export interface AuthUser {
  id: string
  name: string
  roles: UserRole[]
}

interface AuthState {
  token: string
  user: AuthUser | null
}

function readToken(): string {
  return localStorage.getItem(TOKEN_KEY) ?? ''
}

function readUser(): AuthUser | null {
  const raw = localStorage.getItem(USER_KEY)
  if (!raw) {
    return null
  }

  try {
    return JSON.parse(raw) as AuthUser
  } catch {
    localStorage.removeItem(USER_KEY)
    return null
  }
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    token: readToken(),
    user: readUser()
  }),
  getters: {
    isAuthenticated: (state) => Boolean(state.token),
    isAdmin: (state) => state.user?.roles.includes('admin') ?? false,
    roles: (state) => state.user?.roles ?? []
  },
  actions: {
    setToken(token: string) {
      this.token = token
      localStorage.setItem(TOKEN_KEY, token)
    },
    setUser(user: AuthUser | null) {
      this.user = user
      if (user) {
        localStorage.setItem(USER_KEY, JSON.stringify(user))
      } else {
        localStorage.removeItem(USER_KEY)
      }
    },
    setAuth(token: string, user: AuthUser) {
      this.setToken(token)
      this.setUser(user)
    },
    clearAuth() {
      this.token = ''
      this.user = null
      localStorage.removeItem(TOKEN_KEY)
      localStorage.removeItem(USER_KEY)
    }
  }
})
