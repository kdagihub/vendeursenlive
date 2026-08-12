import { createPinia } from 'pinia'
import { mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import ToastService from 'primevue/toastservice'

import AuthDialog from '@/components/auth/AuthDialog.vue'
import { resolveAuthDestination, safeInternalRedirect } from '@/composables/useAuthDialog'

vi.mock('@/services/api', () => ({
  API_BASE_URL: 'https://api.vendeursenlive.shop',
  api: {
    get: vi.fn<(url: string) => Promise<{ data: unknown }>>(),
    patch: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown }>>(),
    post: vi.fn<(url: string, payload?: unknown) => Promise<{ data: unknown }>>(),
  },
  ensureCsrfToken: vi.fn<() => Promise<void>>(),
}))

const dialogStub = {
  props: ['visible'],
  template: '<div v-if="visible" role="dialog"><slot /></div>',
}

async function mountDialog(path: string) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: { template: '<div />' } },
      { path: '/terms', component: { template: '<div />' } },
      { path: '/privacy', component: { template: '<div />' } },
      { path: '/reset-password', component: { template: '<div />' } },
      { path: '/profile', component: { template: '<div />' } },
    ],
  })
  await router.push(path)
  await router.isReady()

  const wrapper = mount(AuthDialog, {
    global: {
      plugins: [createPinia(), router, ToastService],
      stubs: {
        Button: { template: '<button><slot /></button>' },
        Dialog: dialogStub,
        InputText: { template: '<input />' },
        Message: { template: '<div><slot /></div>' },
        Password: { template: '<input type="password" />' },
      },
    },
  })

  return { router, wrapper }
}

describe('authentication dialog', () => {
  beforeEach(() => vi.clearAllMocks())

  it('starts with method choices and no credential field', async () => {
    const { wrapper } = await mountDialog('/?auth=login')

    expect(wrapper.text()).toContain('Utiliser un téléphone ou un email')
    expect(wrapper.text()).toContain('Continuer avec Google')
    expect(wrapper.find('input').exists()).toBe(false)
  })

  it('opens phone and email choices inside the same dialog', async () => {
    const { wrapper } = await mountDialog('/?auth=login')

    await wrapper.get('.auth-provider-button').trigger('click')

    expect(wrapper.text()).toContain('Téléphone')
    expect(wrapper.text()).toContain('Email')
    expect(wrapper.text()).toContain('Numéro de téléphone')
    expect(wrapper.find('input').exists()).toBe(true)
  })

  it('asks for the account type before registration credentials', async () => {
    const { wrapper } = await mountDialog('/?auth=register')

    await wrapper.get('.auth-provider-button').trigger('click')

    expect(wrapper.text()).toContain('Je suis client')
    expect(wrapper.text()).toContain('Je suis vendeur')
    expect(wrapper.find('input').exists()).toBe(false)
  })

  it('returns to the live wall by default and accepts only internal redirects', () => {
    expect(resolveAuthDestination(undefined)).toBe('/')
    expect(resolveAuthDestination('/?live=connect')).toBe('/?live=connect')
    expect(resolveAuthDestination('/app')).toBe('/')
    expect(resolveAuthDestination('/profile')).toBe('/')
    expect(resolveAuthDestination('/profile?tab=orders')).toBe('/')
    expect(resolveAuthDestination('/live/demo')).toBe('/live/demo')
    expect(safeInternalRedirect('https://example.com')).toBeNull()
    expect(safeInternalRedirect('//example.com')).toBeNull()
  })
})
