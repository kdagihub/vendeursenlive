import { beforeEach, describe, expect, it } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'

import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

describe('live discovery store', () => {
  beforeEach(() => setActivePinia(createPinia()))

  it('filters lives by product, seller or location', () => {
    const store = useLiveDiscoveryStore()
    store.searchQuery = 'Toyota'

    expect(store.visibleCategories).toHaveLength(1)
    expect(store.visibleCategories[0]?.id).toBe('vehicles')
    expect(store.visibleCategories[0]?.lives[0]?.sellerName).toBe('Auto Select')
  })

  it('keeps only the selected category', () => {
    const store = useLiveDiscoveryStore()
    store.activeCategory = 'beauty'

    expect(store.visibleCategories.map((category) => category.id)).toEqual(['beauty'])
  })

  it('navigates circularly between live rooms', () => {
    const store = useLiveDiscoveryStore()

    expect(store.adjacentLive('live-awa-style', 'next')?.id).toBe('live-chez-nadia')
    expect(store.adjacentLive('live-awa-style', 'previous')?.id).toBe('live-maison-k')
  })
})
