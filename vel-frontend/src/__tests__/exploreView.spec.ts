import { createPinia, setActivePinia } from 'pinia'
import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import ExploreView from '@/views/ExploreView.vue'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

const responsiveLayoutStub = {
  template: '<div><slot /></div>',
}

const liveCardStub = {
  props: ['live'],
  template: '<div class="live-result">{{ live.sellerName }}</div>',
}

describe('explore view', () => {
  it('searches across sellers, products and locations', async () => {
    const pinia = createPinia()
    setActivePinia(pinia)
    const discovery = useLiveDiscoveryStore()

    const wrapper = mount(ExploreView, {
      global: {
        plugins: [pinia],
        stubs: {
          LiveCard: liveCardStub,
          ResponsiveLayout: responsiveLayoutStub,
        },
      },
    })

    expect(wrapper.text()).toContain('6 résultats')

    discovery.searchQuery = 'Toyota'
    await wrapper.vm.$nextTick()

    expect(wrapper.text()).toContain('1 résultat')
    expect(wrapper.get('.live-result').text()).toBe('Auto Select')
  })
})
