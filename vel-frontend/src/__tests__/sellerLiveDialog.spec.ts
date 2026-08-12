import { createPinia, setActivePinia } from 'pinia'
import { flushPromises, mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const { get, post, ensureCsrfToken } = vi.hoisted(() => ({
  get: vi.fn<(url: string) => Promise<{ data: unknown; status: number }>>(),
  post: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown; status: number }>>(),
  ensureCsrfToken: vi.fn<() => Promise<void>>(),
}))

vi.mock('@/services/api', () => ({
  API_BASE_URL: 'https://api.vendeursenlive.shop',
  api: { get, post },
  ensureCsrfToken,
}))

import SellerLiveDialog from '@/components/SellerLiveDialog.vue'
import { useAuthStore, type AuthUser } from '@/stores/auth'

const dialogStub = {
  props: ['visible'],
  template: '<div v-if="visible" role="dialog"><slot /></div>',
}

async function mountDialog(user: AuthUser) {
  const pinia = createPinia()
  setActivePinia(pinia)
  useAuthStore().user = user

  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: { template: '<div />' } },
      { path: '/profile', component: { template: '<div />' } },
    ],
  })
  await router.push('/?live=connect')
  await router.isReady()

  const wrapper = mount(SellerLiveDialog, {
    global: {
      plugins: [pinia, router],
      stubs: {
        Button: { template: '<button><slot /></button>' },
        Dialog: dialogStub,
        InputText: { template: '<input />' },
        Message: { template: '<div><slot /></div>' },
      },
    },
  })
  await flushPromises()
  return wrapper
}

describe('seller LIVE dialog', () => {
  beforeEach(() => {
    get.mockReset()
    post.mockReset()
    ensureCsrfToken.mockReset()
  })

  it('shows the LIVE link form to a complete verified seller', async () => {
    get.mockResolvedValueOnce({ data: undefined, status: 204 })
    const wrapper = await mountDialog({
      user_id: 'seller-id',
      is_seller: true,
      is_admin: false,
      account_verified: true,
      shop_name: 'Maison Sigma',
      member_since: '2026-07-01T00:00:00Z',
    })

    expect(wrapper.text()).toContain('Connecter mon LIVE')
    expect(wrapper.text()).toContain('Lien partagé du LIVE TikTok')
    expect(wrapper.text()).toContain('Maison Sigma')
    expect(get).toHaveBeenCalledWith('/seller/lives/current')
  })

  it('invites a connected customer to create a seller account', async () => {
    const wrapper = await mountDialog({
      user_id: 'customer-id',
      is_seller: false,
      is_admin: false,
      account_verified: true,
      member_since: '2026-07-01T00:00:00Z',
    })

    expect(wrapper.text()).toContain('Réservé aux comptes vendeurs')
    expect(wrapper.text()).toContain('Devenir vendeur en LIVE')
    expect(wrapper.text()).not.toContain('Lien partagé du LIVE TikTok')
  })
})
