import { createPinia } from 'pinia'
import { flushPromises, mount } from '@vue/test-utils'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import BottomMenu from '@/components/BottomMenu.vue'

describe('mobile bottom menu', () => {
  it('places the LIVE action between the four navigation items', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { template: '<div />' } },
        { path: '/explore', component: { template: '<div />' } },
        { path: '/profile', component: { template: '<div />' } },
      ],
    })
    await router.push('/')
    await router.isReady()

    const wrapper = mount(BottomMenu, {
      global: { plugins: [createPinia(), router] },
    })

    const labels = wrapper.findAll('nav > *').map((item) => item.text())
    expect(labels).toEqual(['Accueil', 'Explorer', 'LIVE', 'Commandes', 'Profil'])

    await wrapper.get('button[aria-label="Connecter mon LIVE"]').trigger('click')
    await flushPromises()
    expect(router.currentRoute.value.query).toMatchObject({
      auth: 'register',
      redirect: '/?live=connect',
    })
  })
})
