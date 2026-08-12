import { createPinia } from 'pinia'
import { mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import AccountMenu from '@/components/AccountMenu.vue'
import { useAuthStore } from '@/stores/auth'

describe('account menu', () => {
  it('reveals profile and logout actions when clicked', async () => {
    const pinia = createPinia()
    const auth = useAuthStore(pinia)
    auth.user = {
      user_id: 'user-1',
      is_seller: false,
      is_admin: false,
      account_verified: true,
      full_name: 'Awa Kouamé',
    }

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { template: '<div />' } },
        { path: '/profile', component: { template: '<div />' } },
      ],
    })
    await router.push('/')
    await router.isReady()

    const wrapper = mount(AccountMenu, {
      global: { plugins: [pinia, router] },
    })

    expect(wrapper.find('[role="menu"]').exists()).toBe(false)
    await wrapper.get('.account-menu-trigger').trigger('click')

    const actions = wrapper.findAll('[role="menuitem"]')
    expect(actions).toHaveLength(2)
    expect(actions[0]?.text()).toContain('Voir le profil')
    expect(actions[1]?.text()).toContain('Déconnexion')
  })
})
