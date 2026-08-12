import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const { get, post, ensureCsrfToken } = vi.hoisted(() => ({
  get: vi.fn<(url: string) => Promise<{ data: unknown; status: number }>>(),
  post: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown; status: number }>>(),
  ensureCsrfToken: vi.fn<() => Promise<void>>(),
}))

vi.mock('@/services/api', () => ({
  api: { get, post },
  ensureCsrfToken,
}))

import { useSellerLiveStore } from '@/stores/sellerLive'

const liveSession = {
  id: '019f-live',
  tiktok_live_url: 'https://www.tiktok.com/@vendeursenlive/live',
  tiktok_username: 'vendeursenlive',
  status: 'ongoing' as const,
  created_at: '2026-08-03T10:00:00Z',
  ended_at: null,
}

describe('seller LIVE store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    get.mockReset()
    post.mockReset()
    ensureCsrfToken.mockReset()
  })

  it('starts a LIVE behind CSRF protection', async () => {
    post.mockResolvedValueOnce({ data: liveSession, status: 201 })
    const store = useSellerLiveStore()

    await store.start('https://vm.tiktok.com/example')

    expect(ensureCsrfToken).toHaveBeenCalledOnce()
    expect(post).toHaveBeenCalledWith('/seller/lives', {
      tiktok_live_url: 'https://vm.tiktok.com/example',
    })
    expect(store.current?.tiktok_username).toBe('vendeursenlive')
  })

  it('clears the current session after the seller ends it', async () => {
    post
      .mockResolvedValueOnce({ data: liveSession, status: 201 })
      .mockResolvedValueOnce({ data: { ...liveSession, status: 'ended' }, status: 200 })
    const store = useSellerLiveStore()
    await store.start(liveSession.tiktok_live_url)

    await store.end()

    expect(post).toHaveBeenLastCalledWith('/seller/lives/019f-live/end')
    expect(store.current).toBeNull()
  })
})
