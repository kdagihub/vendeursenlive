<script setup lang="ts">
import { ArrowRight, Search, SlidersHorizontal } from '@lucide/vue'
import { computed } from 'vue'

import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'
import LiveCard from '@/components/LiveCard.vue'

const discovery = useLiveDiscoveryStore()
const heroLive = computed(() => discovery.featuredLives[0])
</script>

<template>
  <div class="mx-auto w-full max-w-[1500px] px-4 py-5 sm:px-6 lg:px-8 lg:py-8">
    <section id="mobile-search" class="mb-5 lg:hidden">
      <label
        class="flex h-11 items-center gap-3 rounded-md border border-slate-300 bg-white px-3 dark:border-white/15 dark:bg-white/8"
      >
        <Search :size="19" class="text-slate-500" />
        <input
          v-model="discovery.searchQuery"
          class="min-w-0 flex-1 bg-transparent text-sm outline-none dark:text-white"
          placeholder="Produit, vendeur, commune"
          type="search"
        />
        <SlidersHorizontal :size="19" class="text-slate-500" />
      </label>
    </section>

    <section
      v-if="heroLive"
      class="relative mb-9 hidden min-h-[360px] overflow-hidden rounded-md bg-[#17121f] lg:block"
      aria-labelledby="featured-title"
    >
      <video
        class="absolute inset-0 h-full w-full object-cover opacity-75"
        :src="heroLive.videoUrl"
        autoplay
        loop
        muted
        playsinline
      />
      <div class="absolute inset-0 bg-gradient-to-r from-black via-black/50 to-transparent" />
      <div class="relative flex min-h-[360px] max-w-xl flex-col justify-end p-9 text-white">
        <span
          class="mb-4 inline-flex w-fit items-center gap-2 bg-[#ff3447] px-3 py-1.5 text-xs font-black uppercase"
        >
          <span class="h-2 w-2 animate-pulse rounded-full bg-white" />
          À la une maintenant
        </span>
        <h1 id="featured-title" class="text-4xl leading-tight font-black">{{ heroLive.title }}</h1>
        <p class="mt-3 text-base text-white/80">
          {{ heroLive.sellerName }} présente {{ heroLive.product.name.toLowerCase() }} depuis
          {{ heroLive.location }}.
        </p>
        <RouterLink
          :to="`/live/${heroLive.id}`"
          class="mt-6 inline-flex h-11 w-fit items-center gap-2 rounded-md bg-white px-5 text-sm font-black text-[#17121f] no-underline hover:bg-slate-100"
        >
          Entrer dans le live
          <ArrowRight :size="18" />
        </RouterLink>
      </div>
    </section>

    <section id="lives" aria-labelledby="lives-title">
      <div class="mb-5 flex items-end justify-between gap-4">
        <div>
          <p class="text-xs font-black text-[#d92035] uppercase">En ce moment</p>
          <h2
            id="lives-title"
            class="mt-1 text-2xl font-black text-[#211536] lg:text-3xl dark:text-white"
          >
            Les vendeurs sont en direct
          </h2>
        </div>
        <span class="hidden text-sm text-slate-500 sm:block">Mis à jour en temps réel</span>
      </div>

      <div class="-mx-4 mb-7 flex gap-2 overflow-x-auto px-4 pb-1 sm:mx-0 sm:px-0">
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
          Tout voir
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

      <div v-if="discovery.visibleCategories.length" class="space-y-9 lg:space-y-11">
        <section v-for="category in discovery.visibleCategories" :key="category.id">
          <div class="mb-3 flex items-end justify-between gap-4">
            <div>
              <h3 class="text-lg font-black text-[#211536] lg:text-xl dark:text-white">
                {{ category.label }}
              </h3>
              <p class="mt-0.5 text-sm text-slate-500 dark:text-slate-400">
                {{ category.description }}
              </p>
            </div>
            <button
              class="hidden items-center gap-1 text-sm font-bold text-[#d92035] sm:flex"
              type="button"
            >
              Voir tout <ArrowRight :size="16" />
            </button>
          </div>
          <div
            class="-mx-4 flex snap-x snap-mandatory gap-3 overflow-x-auto px-4 pb-2 sm:mx-0 sm:px-0 lg:gap-4"
          >
            <LiveCard
              v-for="(live, index) in category.lives"
              :key="live.id"
              class="snap-start"
              :live="live"
              :priority="index === 0"
            />
          </div>
        </section>
      </div>

      <div v-else class="border-y border-slate-200 py-16 text-center dark:border-white/10">
        <p class="text-lg font-black">Aucun live trouvé</p>
        <p class="mt-1 text-sm text-slate-500">Essaie un autre vendeur, produit ou filtre.</p>
      </div>
    </section>
  </div>
</template>
