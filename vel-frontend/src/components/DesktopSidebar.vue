<script setup lang="ts">
import { CarFront, Compass, House, LayoutGrid, Shirt, ShoppingBasket, Sparkles } from '@lucide/vue'

import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

const discovery = useLiveDiscoveryStore()

const categoryIcons = {
  mode: Shirt,
  beauty: Sparkles,
  food: ShoppingBasket,
  vehicles: CarFront,
  home: House,
}
</script>

<template>
  <aside
    class="fixed top-18 bottom-0 left-0 z-30 hidden w-64 flex-col bg-transparent px-3 py-5 lg:flex"
  >
    <nav class="space-y-1" aria-label="Navigation découverte">
      <RouterLink
        class="flex h-11 items-center gap-3 px-3 font-bold text-[#d92035] no-underline"
        to="/"
      >
        <House :size="20" />
        Accueil
      </RouterLink>
      <a
        class="flex h-11 items-center gap-3 rounded-md px-3 font-semibold text-slate-700 no-underline hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-white/8"
        href="#lives"
      >
        <Compass :size="20" />
        Explorer
      </a>
    </nav>

    <div class="mt-7">
      <p class="px-3 text-xs font-extrabold text-slate-500 uppercase">Catégories</p>
      <div class="mt-2 space-y-1">
        <button
          class="flex h-10 w-full items-center gap-3 rounded-md px-3 text-left text-sm font-semibold transition"
          :class="
            discovery.activeCategory === 'all'
              ? 'bg-slate-950 text-white dark:bg-white dark:text-slate-950'
              : 'text-slate-700 hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-white/8'
          "
          type="button"
          @click="discovery.activeCategory = 'all'"
        >
          <LayoutGrid :size="18" />
          Toutes
        </button>
        <button
          v-for="category in discovery.categories"
          :key="category.id"
          class="flex h-10 w-full items-center gap-3 rounded-md px-3 text-left text-sm font-semibold transition"
          :class="
            discovery.activeCategory === category.id
              ? 'bg-slate-950 text-white dark:bg-white dark:text-slate-950'
              : 'text-slate-700 hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-white/8'
          "
          type="button"
          @click="discovery.activeCategory = category.id"
        >
          <component :is="categoryIcons[category.id]" :size="18" />
          {{ category.label }}
        </button>
      </div>
    </div>

    <div class="mt-auto px-3 pt-5">
      <p class="text-sm font-black text-slate-950 dark:text-white">Tu vends déjà en live ?</p>
      <p class="mt-1 text-xs leading-5 text-slate-500 dark:text-slate-400">
        Centralise tes commandes et garde le contrôle de tes ventes.
      </p>
      <RouterLink
        class="mt-3 inline-flex h-10 w-full items-center justify-center rounded-md bg-[#ff3447] text-sm font-bold text-white no-underline hover:bg-[#d92035]"
        to="/login?mode=register"
      >
        Devenir vendeur
      </RouterLink>
      <nav class="mt-4 flex flex-wrap gap-x-3 gap-y-1 text-[11px]" aria-label="Liens légaux">
        <RouterLink class="text-slate-500 hover:text-[#d92035]" to="/terms">CGU</RouterLink>
        <RouterLink class="text-slate-500 hover:text-[#d92035]" to="/privacy">
          Confidentialité
        </RouterLink>
      </nav>
    </div>
  </aside>
</template>
