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
  can_change_password?: boolean
  verification_channel?: 'email' | 'phone' | null
  full_name?: string | null
  avatar_url?: string | null
  email?: string | null
  phone_number?: string | null
  account_status?: 'active' | 'disabled'
  auth_methods?: Array<'email' | 'phone' | 'google' | 'tiktok'>
  shop_name?: string | null
  default_location?: string | null
  member_since?: string
}

export interface RegisterPayload {
  full_name?: string
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

export interface PhoneOtpRequestPayload {
  phone_number: string
  purpose: 'login' | 'register'
  full_name?: string
  account_type?: AccountType
  shop_name?: string
}

export interface PhoneOtpChallenge {
  challenge_id: string
  expires_in_seconds: number
  resend_after_seconds: number
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
  const displayName = computed(
    () => user.value?.full_name || user.value?.shop_name || accountLabel.value,
  )
  const profileComplete = computed(
    () =>
      Boolean(user.value?.full_name) &&
      (user.value?.is_seller !== true || Boolean(user.value.shop_name)),
  )

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

  async function requestPhoneOtp(payload: PhoneOtpRequestPayload) {
    let challenge: PhoneOtpChallenge | null = null
    await runAuthRequest(async () => {
      const response = await api.post<PhoneOtpChallenge>(
        '/auth/phone/otp/request',
        compactPayload(payload),
      )
      challenge = response.data
    })
    return challenge as PhoneOtpChallenge | null
  }

  async function verifyPhoneOtp(challengeId: string, otp: string) {
    await runAuthRequest(async () => {
      const response = await api.post<AuthUser>('/auth/phone/otp/verify', {
        challenge_id: challengeId,
        otp,
      })
      user.value = response.data
      await ensureCsrfToken()
    })
  }

  function startGoogleOAuth(accountType?: AccountType) {
    window.location.assign(buildGoogleOAuthStartUrl(API_BASE_URL, accountType))
  }

  async function fetchMe(refreshCsrf = true) {
    try {
      const response = await api.get<AuthUser>('/auth/me')
      user.value = response.data
      if (refreshCsrf) await ensureCsrfToken()
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
      await fetchMe(false)
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

  async function updateProfile(fullName?: string, shopName?: string) {
    await runAuthRequest(async () => {
      await ensureCsrfToken()
      await api.patch('/auth/profile', compactPayload({ full_name: fullName, shop_name: shopName }))
      await fetchMe(false)
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
    displayName,
    error,
    fetchMe,
    isAuthenticated,
    loading,
    login,
    logout,
    profileComplete,
    refresh,
    register,
    requestEmailVerification,
    requestPhoneOtp,
    requestPasswordReset,
    requiresEmailVerification,
    startGoogleOAuth,
    user,
    updateProfile,
    verifyPhoneOtp,
  }
})

export function buildGoogleOAuthStartUrl(baseUrl: string, accountType?: AccountType): string {
  const endpoint = new URL(`${baseUrl.replace(/\/$/, '')}/auth/google/start`)
  if (accountType) endpoint.searchParams.set('account_type', accountType)
  return endpoint.toString()
}

function compactPayload<T extends object>(payload: T): Partial<T> {
  return Object.fromEntries(
    Object.entries(payload).filter(([, value]) => value !== undefined && value !== ''),
  ) as Partial<T>
}

function toPublicError(requestError: unknown): string {
  if (axios.isAxiosError(requestError)) {
    const status = requestError.response?.status
    const message = extractMessage(requestError.response?.data)

    if (status === 401) return 'Identifiants ou code de vérification invalides.'
    if (status === 403) return 'La protection CSRF a refusé la requête.'
    if (status === 409) return 'Un compte existe déjà avec ces informations.'
    if (status === 429) return 'Trop de tentatives. Réessaie dans quelques instants.'
    if (status === 503) return 'Le service SMS est temporairement indisponible.'

    if (message) {
      return message
    }
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
