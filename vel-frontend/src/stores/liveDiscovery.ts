import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import type { LiveCategory, LivePreview } from '@/types/live'

const previewVideos = {
  portrait: 'https://interactive-examples.mdn.mozilla.net/media/cc0-videos/flower.mp4',
  landscape: 'https://storage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4',
}

const categories: LiveCategory[] = [
  { id: 'mode', label: 'Mode', description: 'Tenues, chaussures et accessoires' },
  { id: 'beauty', label: 'Beauté', description: 'Soins, parfums et coiffure' },
  { id: 'food', label: 'Repas', description: 'Cuisine et produits gourmands' },
  { id: 'vehicles', label: 'Véhicules', description: 'Autos, motos et équipements' },
  { id: 'home', label: 'Maison', description: 'Décoration et immobilier' },
]

const seedLives: LivePreview[] = [
  {
    id: 'live-awa-style',
    sellerName: 'Awa Style',
    sellerHandle: '@awastyle_ci',
    title: 'Nouvelle collection du vendredi',
    category: 'mode',
    location: 'Cocody',
    viewerCount: 1280,
    videoUrl: previewVideos.portrait,
    tiktokUsername: 'creatoracademy',
    featured: true,
    product: { name: 'Ensemble wax premium', price: 25000 },
  },
  {
    id: 'live-chez-nadia',
    sellerName: 'Chez Nadia',
    sellerHandle: '@cheznadia225',
    title: 'Sacs et chaussures arrivage',
    category: 'mode',
    location: 'Yopougon',
    viewerCount: 846,
    videoUrl: previewVideos.landscape,
    tiktokUsername: 'creatoracademy',
    featured: false,
    product: { name: 'Sac à main grainé', price: 18000 },
  },
  {
    id: 'live-glow-ci',
    sellerName: 'Glow CI',
    sellerHandle: '@glowci',
    title: 'Routine peau lumineuse',
    category: 'beauty',
    location: 'Marcory',
    viewerCount: 612,
    videoUrl: previewVideos.portrait,
    tiktokUsername: 'creatoracademy',
    featured: true,
    product: { name: 'Coffret éclat', price: 14500 },
  },
  {
    id: 'live-saveurs-babi',
    sellerName: 'Saveurs de Babi',
    sellerHandle: '@saveursdebabi',
    title: 'Le menu du soir en direct',
    category: 'food',
    location: 'Plateau',
    viewerCount: 394,
    videoUrl: previewVideos.landscape,
    tiktokUsername: 'creatoracademy',
    featured: true,
    product: { name: 'Pack dîner 2 personnes', price: 12000 },
  },
  {
    id: 'live-auto-select',
    sellerName: 'Auto Select',
    sellerHandle: '@autoselectabj',
    title: 'Visite guidée du parc auto',
    category: 'vehicles',
    location: 'Treichville',
    viewerCount: 257,
    videoUrl: previewVideos.landscape,
    tiktokUsername: 'creatoracademy',
    featured: false,
    product: { name: 'Toyota Corolla 2018', price: 7350000 },
  },
  {
    id: 'live-maison-k',
    sellerName: 'Maison K',
    sellerHandle: '@maisonk_ci',
    title: 'Déco salon et bonnes affaires',
    category: 'home',
    location: 'Bingerville',
    viewerCount: 188,
    videoUrl: previewVideos.portrait,
    tiktokUsername: 'creatoracademy',
    featured: false,
    product: { name: 'Lampe de salon', price: 32000 },
  },
]

export const useLiveDiscoveryStore = defineStore('live-discovery', () => {
  const lives = ref<LivePreview[]>(seedLives)
  const searchQuery = ref('')
  const activeCategory = ref<string>('all')

  const featuredLives = computed(() => lives.value.filter((live) => live.featured))
  const allLives = computed(() => lives.value)
  const visibleCategories = computed(() =>
    categories
      .filter((category) => activeCategory.value === 'all' || category.id === activeCategory.value)
      .map((category) => ({
        ...category,
        lives: lives.value.filter((live) => {
          const query = searchQuery.value.trim().toLocaleLowerCase('fr')
          const matchesCategory = live.category === category.id
          const matchesQuery =
            query.length === 0 ||
            [live.title, live.sellerName, live.product.name, live.location]
              .join(' ')
              .toLocaleLowerCase('fr')
              .includes(query)

          return matchesCategory && matchesQuery
        }),
      }))
      .filter((category) => category.lives.length > 0),
  )

  function findLive(id: string) {
    return lives.value.find((live) => live.id === id)
  }

  function adjacentLive(id: string, direction: 'previous' | 'next') {
    const currentIndex = lives.value.findIndex((live) => live.id === id)
    if (currentIndex === -1 || lives.value.length === 0) return undefined

    const offset = direction === 'next' ? 1 : -1
    const targetIndex = (currentIndex + offset + lives.value.length) % lives.value.length
    return lives.value[targetIndex]
  }

  return {
    activeCategory,
    adjacentLive,
    allLives,
    categories,
    featuredLives,
    findLive,
    searchQuery,
    visibleCategories,
  }
})
