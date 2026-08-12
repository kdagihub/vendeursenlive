import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

const { post, patch, get, ensureCsrfToken } = vi.hoisted(() => ({
  post: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown }>>(),
  patch: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown }>>(),
  get: vi.fn<(url: string) => Promise<{ data: unknown }>>(),
  ensureCsrfToken: vi.fn<() => Promise<void>>(),
}))

vi.mock('@/services/api', () => ({
  API_BASE_URL: 'https://api.vendeursenlive.shop',
  api: {
    get,
    patch,
    post,
  },
  ensureCsrfToken,
}))

import { buildGoogleOAuthStartUrl, useAuthStore } from '@/stores/auth'

describe('phone OTP authentication store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    post.mockReset()
    patch.mockReset()
    get.mockReset()
    ensureCsrfToken.mockReset()
  })

  it('requests an opaque phone challenge', async () => {
    post.mockResolvedValueOnce({
      data: {
        challenge_id: 'challenge-id',
        expires_in_seconds: 300,
        resend_after_seconds: 60,
      },
    })
    const auth = useAuthStore()

    const challenge = await auth.requestPhoneOtp({
      phone_number: '+2250700000000',
      purpose: 'login',
    })

    expect(post).toHaveBeenCalledWith('/auth/phone/otp/request', {
      phone_number: '+2250700000000',
      purpose: 'login',
    })
    expect(challenge?.challenge_id).toBe('challenge-id')
  })

  it('stores the authenticated user after OTP verification', async () => {
    post.mockResolvedValueOnce({
      data: {
        user_id: 'user-id',
        is_seller: false,
        is_admin: false,
        account_verified: true,
      },
    })
    const auth = useAuthStore()

    await auth.verifyPhoneOtp('challenge-id', '629185')

    expect(post).toHaveBeenCalledWith('/auth/phone/otp/verify', {
      challenge_id: 'challenge-id',
      otp: '629185',
    })
    expect(auth.user?.account_verified).toBe(true)
    expect(ensureCsrfToken).toHaveBeenCalledOnce()
  })

  it('updates optional profile information behind CSRF protection', async () => {
    patch.mockResolvedValueOnce({ data: undefined })
    get.mockResolvedValueOnce({
      data: {
        user_id: 'user-id',
        is_seller: true,
        is_admin: false,
        account_verified: true,
        full_name: 'Awa Kouamé',
        shop_name: 'Boutique Awa',
        auth_methods: ['phone'],
        member_since: '2026-08-04T10:00:00Z',
      },
    })
    const auth = useAuthStore()

    await auth.updateProfile(undefined, 'Boutique Awa')

    expect(ensureCsrfToken).toHaveBeenCalledOnce()
    expect(patch).toHaveBeenCalledWith('/auth/profile', { shop_name: 'Boutique Awa' })
    expect(get).toHaveBeenCalledWith('/auth/me')
    expect(auth.displayName).toBe('Awa Kouamé')
    expect(auth.profileComplete).toBe(true)
  })

  it('builds the Google OAuth start URL without exposing credentials', () => {
    expect(buildGoogleOAuthStartUrl('https://api.vendeursenlive.shop/', 'seller')).toBe(
      'https://api.vendeursenlive.shop/auth/google/start?account_type=seller',
    )
  })
})
