import { describe, it, expect } from 'vitest'

import { mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import ToastService from 'primevue/toastservice'
import App from '../App.vue'

describe('App', () => {
  it('mounts renders properly', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { template: '<div />' } },
        { path: '/login', component: { template: '<div />' } },
        { path: '/terms', component: { template: '<div />' } },
        { path: '/privacy', component: { template: '<div />' } },
        { path: '/profile', component: { template: '<div />' } },
      ],
    })
    await router.push('/login')
    await router.isReady()

    const wrapper = mount(App, {
      global: {
        plugins: [createPinia(), router, ToastService],
        stubs: {
          Button: true,
          Dialog: true,
          Message: true,
          Toast: true,
          RouterView: true,
        },
      },
    })

    expect(wrapper.text()).toContain('VendeursEnLive')
  })
})
