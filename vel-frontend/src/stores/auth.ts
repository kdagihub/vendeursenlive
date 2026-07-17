import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import axios from 'axios'

import { API_BASE_URL, api, ensureCsrfToken } from '@/services/api'

export type AccountType = 'customer' | 'seller'

export interface AuthUser {
  user_id: string
  session_id?: string
  token_type?: string
  expires_in_seconds?: number
  is_seller: boolean
  is_admin: boolean
  account_verified: boolean
  verification_channel?: 'email' | 'phone' | null
}

export interface RegisterPayload {
  full_name: string
  email?: string
  phone_number?: string
  password: string
  account_type: AccountType
  default_location?: string
  shop_name?: string
  payment_link?: string
}

export interface LoginPayload {
  identifier: string
  password: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<AuthUser | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const isAuthenticated = computed(() => user.value !== null)
  const requiresEmailVerification = computed(
    () => user.value?.account_verified === false && user.value.verification_channel === 'email',
  )
  const accountLabel = computed(() => {
    if (!user.value) return 'Invite'
    if (user.value.is_admin) return 'Administration'
    if (user.value.is_seller) return 'Vendeur'
    return 'Client'
  })

  async function register(payload: RegisterPayload) {
    await runAuthRequest(async () => {
      const response = await api.post<AuthUser>('/auth/register', compactPayload(payload))
      user.value = response.data
      await ensureCsrfToken()
    })
  }

  async function login(payload: LoginPayload) {
    await runAuthRequest(async () => {
      const response = await api.post<AuthUser>('/auth/login', payload)
      user.value = response.data
      await ensureCsrfToken()
    })
  }

  async function fetchMe() {
    try {
      const response = await api.get<AuthUser>('/auth/me')
      user.value = response.data
      await ensureCsrfToken()
      return true
    } catch {
      user.value = null
      return false
    }
  }

  async function refresh() {
    await runAuthRequest(async () => {
      await ensureCsrfToken()
      const response = await api.post<AuthUser>('/auth/refresh')
      user.value = response.data
    })
  }

  async function logout() {
    loading.value = true
    error.value = null

    try {
      await ensureCsrfToken()
      await api.post('/auth/logout')
    } catch (requestError) {
      error.value = toPublicError(requestError)
    } finally {
      user.value = null
      loading.value = false
    }
  }

  async function changePassword(currentPassword: string, newPassword: string) {
    await runAuthRequest(async () => {
      await ensureCsrfToken()
      await api.post('/auth/change-password', {
        current_password: currentPassword,
        new_password: newPassword,
      })
    })
  }

  async function requestPasswordReset(email: string) {
    await runAuthRequest(async () => {
      await api.post('/auth/password-reset/request', { email })
    })
  }

  async function confirmPasswordReset(resetToken: string, newPassword: string) {
    await runAuthRequest(async () => {
      await api.post('/auth/password-reset/confirm', {
        reset_token: resetToken,
        new_password: newPassword,
      })
    })
  }

  async function requestEmailVerification() {
    await runAuthRequest(async () => {
      await ensureCsrfToken()
      await api.post('/auth/email-verification/request')
    })
  }

  async function confirmEmailVerification(verificationToken: string) {
    await runAuthRequest(async () => {
      await api.post('/auth/email-verification/confirm', {
        verification_token: verificationToken,
      })

      try {
        await ensureCsrfToken()
        const response = await api.post<AuthUser>('/auth/refresh')
        user.value = response.data
      } catch {
        // The link may be opened on another device where no auth session exists.
        user.value = null
      }
    })
  }

  function startTikTokLogin() {
    window.location.assign(`${API_BASE_URL}/auth/tiktok/start`)
  }

  async function runAuthRequest(action: () => Promise<void>) {
    loading.value = true
    error.value = null

    try {
      await action()
    } catch (requestError) {
      error.value = toPublicError(requestError)
      throw requestError
    } finally {
      loading.value = false
    }
  }

  return {
    accountLabel,
    changePassword,
    confirmPasswordReset,
    confirmEmailVerification,
    error,
    fetchMe,
    isAuthenticated,
    loading,
    login,
    logout,
    refresh,
    register,
    requestEmailVerification,
    requestPasswordReset,
    requiresEmailVerification,
    startTikTokLogin,
    user,
  }
})

function compactPayload<T extends object>(payload: T): Partial<T> {
  return Object.fromEntries(
    Object.entries(payload).filter(([, value]) => value !== undefined && value !== ''),
  ) as Partial<T>
}

function toPublicError(requestError: unknown): string {
  if (axios.isAxiosError(requestError)) {
    const status = requestError.response?.status
    const message = extractMessage(requestError.response?.data)

    if (message) {
      return message
    }

    if (status === 401) return 'Identifiants invalides ou session expirée.'
    if (status === 403) return 'La protection CSRF a refusé la requête.'
    if (status === 409) return 'Un compte existe déjà avec ces informations.'
    if (status === 429) return 'Trop de tentatives. Réessaie dans quelques instants.'
  }

  return 'Une erreur est survenue. Réessaie dans quelques instants.'
}

function extractMessage(data: unknown): string | null {
  if (!data || typeof data !== 'object') {
    return null
  }

  const payload = data as Record<string, unknown>
  const message = payload.message ?? payload.error

  return typeof message === 'string' ? message : null
}
