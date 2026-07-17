<script setup lang="ts">
import { Search } from '@lucide/vue'

import tiktokLogo from '@/assets/img/tiktok-logo_sf.png'
import BrandMark from '@/components/BrandMark.vue'
import { useAuthStore } from '@/stores/auth'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

const auth = useAuthStore()
const discovery = useLiveDiscoveryStore()
</script>

<template>
  <header class="fixed inset-x-0 top-0 z-40 hidden h-18 items-center bg-transparent px-6 lg:flex">
    <div class="w-64 shrink-0">
      <BrandMark />
    </div>

    <label class="mx-auto flex w-full max-w-2xl items-center gap-3" aria-label="Rechercher">
      <Search :size="20" class="pointer-events-none -mr-11 z-10 text-slate-500" />
      <input
        v-model="discovery.searchQuery"
        class="h-11 w-full rounded-md border border-transparent bg-white/75 pr-4 pl-12 text-sm text-slate-950 shadow-sm outline-none backdrop-blur transition focus:border-[#ff3447]/40 focus:bg-white focus:ring-2 focus:ring-[#ff3447]/15 dark:bg-white/8 dark:text-white dark:placeholder:text-slate-400 dark:focus:bg-white/12"
        placeholder="Chercher un produit, un vendeur ou une commune"
        type="search"
      />
    </label>

    <div class="ml-6 flex w-80 shrink-0 items-center justify-end gap-2.5">
      <template v-if="auth.isAuthenticated">
        <RouterLink
          class="inline-flex h-10 items-center rounded-md bg-[#ff3447] px-4 text-sm font-bold text-white no-underline hover:bg-[#d92035]"
          to="/app"
        >
          Mon espace
        </RouterLink>
      </template>
      <template v-else>
        <RouterLink
          class="inline-flex h-10 items-center rounded-md border border-slate-300 px-4 text-sm font-bold text-slate-800 no-underline hover:border-slate-500 dark:border-white/20 dark:text-white"
          to="/login"
        >
          Se connecter
        </RouterLink>
        <button
          class="inline-flex h-10 items-center gap-2 rounded-md bg-[#111] px-4 text-sm font-bold text-white hover:bg-black"
          type="button"
          @click="auth.startTikTokLogin"
        >
          <img :src="tiktokLogo" alt="" class="h-5 w-5 object-contain" />
          TikTok
        </button>
      </template>
    </div>
  </header>
</template>
