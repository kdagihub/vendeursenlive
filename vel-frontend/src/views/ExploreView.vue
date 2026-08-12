<script setup lang="ts">
import { MapPin, Search, SlidersHorizontal, X } from '@lucide/vue'
import { computed, ref } from 'vue'

import LiveCard from '@/components/LiveCard.vue'
import ResponsiveLayout from '@/layouts/ResponsiveLayout.vue'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

type SortMode = 'popular' | 'seller'

const discovery = useLiveDiscoveryStore()
const selectedLocation = ref('all')
const sortMode = ref<SortMode>('popular')

const locations = computed(() =>
  [...new Set(discovery.allLives.map((live) => live.location))].sort((a, b) =>
    a.localeCompare(b, 'fr'),
  ),
)

const filteredLives = computed(() => {
  const query = discovery.searchQuery.trim().toLocaleLowerCase('fr')
  const result = discovery.allLives.filter((live) => {
    const matchesQuery =
      !query ||
      [live.title, live.sellerName, live.sellerHandle, live.product.name, live.location]
        .join(' ')
        .toLocaleLowerCase('fr')
        .includes(query)
    const matchesCategory =
      discovery.activeCategory === 'all' || live.category === discovery.activeCategory
    const matchesLocation =
      selectedLocation.value === 'all' || live.location === selectedLocation.value

    return matchesQuery && matchesCategory && matchesLocation
  })

  return [...result].sort((a, b) =>
    sortMode.value === 'popular'
      ? b.viewerCount - a.viewerCount
      : a.sellerName.localeCompare(b.sellerName, 'fr'),
  )
})

const hasFilters = computed(
  () =>
    discovery.searchQuery.trim().length > 0 ||
    discovery.activeCategory !== 'all' ||
    selectedLocation.value !== 'all',
)

function clearFilters() {
  discovery.searchQuery = ''
  discovery.activeCategory = 'all'
  selectedLocation.value = 'all'
  sortMode.value = 'popular'
}
</script>

<template>
  <ResponsiveLayout>
    <div class="mx-auto w-full max-w-[1500px] px-4 py-5 sm:px-6 lg:px-8 lg:py-8">
      <header class="mb-6 lg:mb-8">
        <p class="text-xs font-black text-[#d92035] uppercase">Découverte</p>
        <h1 class="mt-1 text-2xl font-black text-[#211536] lg:text-3xl dark:text-white">
          Explorer les lives
        </h1>
        <p class="mt-1 max-w-2xl text-sm text-slate-500 dark:text-slate-400">
          Recherche un article, un vendeur ou une commune parmi les directs disponibles.
        </p>
      </header>

      <section class="mb-7" aria-label="Recherche et filtres">
        <label
          id="mobile-search"
          class="flex h-12 w-full items-center gap-3 rounded-md border border-slate-300 bg-white px-4 shadow-sm focus-within:border-[#ff3447]/50 focus-within:ring-2 focus-within:ring-[#ff3447]/10 lg:max-w-3xl dark:border-white/15 dark:bg-white/8"
        >
          <Search :size="20" class="shrink-0 text-slate-500" aria-hidden="true" />
          <input
            v-model="discovery.searchQuery"
            class="min-w-0 flex-1 bg-transparent text-sm text-slate-950 outline-none dark:text-white dark:placeholder:text-slate-400"
            placeholder="Produit, vendeur ou commune"
            type="search"
          />
          <button
            v-if="discovery.searchQuery"
            class="grid h-8 w-8 shrink-0 place-items-center rounded-full text-slate-500 hover:bg-slate-100 dark:hover:bg-white/10"
            type="button"
            aria-label="Effacer la recherche"
            @click="discovery.searchQuery = ''"
          >
            <X :size="17" aria-hidden="true" />
          </button>
        </label>

        <div class="mt-4 flex gap-2 overflow-x-auto pb-1" aria-label="Catégories">
          <button
            class="h-9 shrink-0 rounded-md px-4 text-sm font-bold"
            :class="
              discovery.activeCategory === 'all'
                ? 'bg-[#17121f] text-white dark:bg-white dark:text-[#17121f]'
                : 'border border-slate-300 bg-white text-slate-700 dark:border-white/15 dark:bg-white/5 dark:text-slate-200'
            "
            type="button"
            @click="discovery.activeCategory = 'all'"
          >
            Toutes
          </button>
          <button
            v-for="category in discovery.categories"
            :key="category.id"
            class="h-9 shrink-0 rounded-md px-4 text-sm font-bold"
            :class="
              discovery.activeCategory === category.id
                ? 'bg-[#17121f] text-white dark:bg-white dark:text-[#17121f]'
                : 'border border-slate-300 bg-white text-slate-700 dark:border-white/15 dark:bg-white/5 dark:text-slate-200'
            "
            type="button"
            @click="discovery.activeCategory = category.id"
          >
            {{ category.label }}
          </button>
        </div>

        <div
          class="-mx-4 mt-3 flex flex-nowrap items-center gap-2 overflow-x-auto px-4 pb-1 sm:mx-0 sm:px-0"
        >
          <label class="relative w-44 shrink-0">
            <MapPin
              :size="17"
              class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-slate-500"
              aria-hidden="true"
            />
            <select
              v-model="selectedLocation"
              class="h-10 w-full appearance-none rounded-md border border-slate-300 bg-white pr-8 pl-9 text-sm font-semibold text-slate-700 outline-none dark:border-white/15 dark:bg-[#19191b] dark:text-slate-200"
              aria-label="Filtrer par commune"
            >
              <option value="all">Toutes les communes</option>
              <option v-for="location in locations" :key="location" :value="location">
                {{ location }}
              </option>
            </select>
          </label>

          <label class="relative w-40 shrink-0">
            <SlidersHorizontal
              :size="17"
              class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-slate-500"
              aria-hidden="true"
            />
            <select
              v-model="sortMode"
              class="h-10 w-full appearance-none rounded-md border border-slate-300 bg-white pr-8 pl-9 text-sm font-semibold text-slate-700 outline-none dark:border-white/15 dark:bg-[#19191b] dark:text-slate-200"
              aria-label="Trier les résultats"
            >
              <option value="popular">Plus populaires</option>
              <option value="seller">Nom du vendeur</option>
            </select>
          </label>

          <button
            v-if="hasFilters"
            class="h-10 shrink-0 px-2 text-sm font-bold text-[#d92035]"
            type="button"
            @click="clearFilters"
          >
            Réinitialiser
          </button>
        </div>
      </section>

      <section aria-labelledby="explore-results-title">
        <div class="mb-4 flex items-end justify-between gap-4">
          <div>
            <h2 id="explore-results-title" class="text-lg font-black lg:text-xl">
              Lives disponibles
            </h2>
            <p class="mt-0.5 text-sm text-slate-500 dark:text-slate-400">
              {{ filteredLives.length }} résultat{{ filteredLives.length === 1 ? '' : 's' }}
            </p>
          </div>
          <span class="live-status-badge"><span aria-hidden="true" />Mis à jour en direct</span>
        </div>

        <div
          v-if="filteredLives.length"
          class="grid grid-cols-[repeat(auto-fill,minmax(min(100%,220px),1fr))] gap-4 lg:gap-5"
        >
          <LiveCard
            v-for="(live, index) in filteredLives"
            :key="live.id"
            fluid
            :live="live"
            :priority="index < 2"
          />
        </div>

        <div v-else class="border-y border-slate-200 py-16 text-center dark:border-white/10">
          <Search :size="30" class="mx-auto text-slate-400" aria-hidden="true" />
          <h3 class="mt-3 text-lg font-black">Aucun live trouvé</h3>
          <p class="mt-1 text-sm text-slate-500">
            Modifie les filtres ou réinitialise la recherche.
          </p>
          <button
            class="mt-5 h-10 rounded-md bg-[#17121f] px-5 text-sm font-bold text-white dark:bg-white dark:text-[#17121f]"
            type="button"
            @click="clearFilters"
          >
            Voir tous les lives
          </button>
        </div>
      </section>
    </div>
  </ResponsiveLayout>
</template>
