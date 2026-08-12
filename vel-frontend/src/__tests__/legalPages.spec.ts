import { mount, shallowMount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import LegalFooter from '@/components/LegalFooter.vue'
import DesktopTopbar from '@/components/DesktopTopbar.vue'
import LegalLayout from '@/layouts/LegalLayout.vue'
import PrivacyView from '@/views/PrivacyView.vue'
import TermsView from '@/views/TermsView.vue'

const legalLayoutStub = {
  template: '<div><slot /></div>',
}

describe('legal pages', () => {
  it('keeps the legal shell and sticky header consistent in dark mode', () => {
    const wrapper = shallowMount(LegalLayout, {
      global: {
        stubs: {
          BrandMark: true,
          LegalFooter: true,
          RouterLink: true,
        },
      },
    })

    expect(wrapper.classes()).toContain('dark:bg-[#0d0d0e]')
    expect(wrapper.classes()).toContain('dark:text-white')
    expect(wrapper.get('header').classes()).toContain('dark:bg-[#0d0d0e]/94')
  })

  it('describes the marketplace order and payment model in the terms', () => {
    const wrapper = shallowMount(TermsView, {
      global: {
        stubs: {
          LegalLayout: legalLayoutStub,
          RouterLink: true,
        },
      },
    })

    expect(wrapper.text()).toContain('Conditions générales d’utilisation')
    expect(wrapper.text()).toContain('arbitrage manuel')
    expect(wrapper.text()).toContain('VendeursEnLive ne reçoit pas les fonds')
    expect(wrapper.text()).toContain('CIACEMS')
    expect(wrapper.text()).toContain('CI-ABJ-03-2024-M-44362')
    expect(wrapper.text()).toContain('KOFFI DJÈCLAY Alexandre')
    expect(wrapper.text()).not.toContain('COULIBALY')
  })

  it('documents phone OTP data and ARTCI rights in the privacy policy', () => {
    const wrapper = shallowMount(PrivacyView, {
      global: {
        stubs: {
          LegalLayout: legalLayoutStub,
        },
      },
    })

    expect(wrapper.text()).toContain('Politique de confidentialité')
    expect(wrapper.text()).toContain('IKODDI')
    expect(wrapper.text()).toContain('cinq minutes')
    expect(wrapper.text()).toContain('ne vend pas les données personnelles')
    expect(wrapper.text()).toContain('ARTCI')
    expect(wrapper.text()).toContain('CI-ABJ-03-2024-M-44362')
    expect(wrapper.text()).toContain('KOFFI DJÈCLAY Alexandre')
    expect(wrapper.text()).not.toContain('COULIBALY')
  })

  it('exposes terms and privacy links from the marketplace footer', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { template: '<div />' } },
        { path: '/terms', component: { template: '<div />' } },
        { path: '/privacy', component: { template: '<div />' } },
      ],
    })
    await router.push('/')
    await router.isReady()

    const wrapper = mount(LegalFooter, {
      global: {
        plugins: [router],
        stubs: {
          BrandMark: true,
        },
      },
    })

    expect(wrapper.get('a[href="/terms"]').text()).toBe('Conditions générales d’utilisation')
    expect(wrapper.get('a[href="/privacy"]').text()).toBe('Politique de confidentialité')
    expect(wrapper.get('a[href^="mailto:"]').attributes('href')).toBe(
      'mailto:contact@vendeursenlive.shop',
    )
    expect(wrapper.get('a[href^="tel:"]').attributes('href')).toBe('tel:+2250797969394')
  })

  it('exposes terms and privacy above the fold in the desktop header', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { template: '<div />' } },
        { path: '/terms', component: { template: '<div />' } },
        { path: '/privacy', component: { template: '<div />' } },
      ],
    })
    await router.push('/')
    await router.isReady()

    const wrapper = mount(DesktopTopbar, {
      global: {
        plugins: [createPinia(), router],
        stubs: { BrandMark: true },
      },
    })

    expect(wrapper.get('a[href="/terms"]').text()).toBe('CGU')
    expect(wrapper.get('a[href="/privacy"]').text()).toBe('Confidentialité')
  })
})
