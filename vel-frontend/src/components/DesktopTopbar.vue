<script setup lang="ts">
import { Search } from '@lucide/vue'
import { ref, watch } from 'vue'
import { useRouter } from 'vue-router'

import AccountMenu from '@/components/AccountMenu.vue'
import BrandMark from '@/components/BrandMark.vue'
import { useAuthDialog } from '@/composables/useAuthDialog'
import { useAuthStore } from '@/stores/auth'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

const auth = useAuthStore()
const discovery = useLiveDiscoveryStore()
const router = useRouter()
const { openAuthDialog } = useAuthDialog()
const searchQuery = ref(discovery.searchQuery)

watch(
  () => discovery.searchQuery,
  (query) => {
    if (query !== searchQuery.value) searchQuery.value = query
  },
)

async function searchLives() {
  discovery.searchQuery = searchQuery.value.trim()
  await router.push('/explore')
}
</script>

<template>
  <header class="fixed inset-x-0 top-0 z-40 hidden h-18 items-center bg-transparent px-6 lg:flex">
    <div class="w-64 shrink-0">
      <BrandMark />
    </div>

    <form
      class="mx-auto flex w-full max-w-2xl items-center gap-3"
      role="search"
      @submit.prevent="searchLives"
    >
      <Search :size="20" class="pointer-events-none -mr-11 z-10 text-slate-500" />
      <input
        v-model="searchQuery"
        class="h-11 w-full rounded-md border border-transparent bg-white/75 pr-4 pl-12 text-sm text-slate-950 shadow-sm outline-none backdrop-blur transition focus:border-[#ff3447]/40 focus:bg-white focus:ring-2 focus:ring-[#ff3447]/15 dark:bg-white/8 dark:text-white dark:placeholder:text-slate-400 dark:focus:bg-white/12"
        placeholder="Chercher un produit, un vendeur ou une commune"
        aria-label="Chercher un produit, un vendeur ou une commune"
        type="search"
      />
    </form>

    <div class="ml-6 flex w-96 shrink-0 items-center justify-end gap-3">
      <nav class="flex items-center gap-3 text-xs font-bold" aria-label="Informations légales">
        <RouterLink
          class="text-slate-600 no-underline hover:text-[#d92035] dark:text-slate-300 dark:hover:text-[#ff6675]"
          to="/terms"
        >
          CGU
        </RouterLink>
        <RouterLink
          class="text-slate-600 no-underline hover:text-[#d92035] dark:text-slate-300 dark:hover:text-[#ff6675]"
          to="/privacy"
        >
          Confidentialité
        </RouterLink>
      </nav>

      <template v-if="auth.isAuthenticated">
        <AccountMenu />
      </template>
      <template v-else>
        <button
          class="inline-flex h-10 items-center rounded-md border border-slate-300 px-4 text-sm font-bold text-slate-800 no-underline hover:border-slate-500 dark:border-white/20 dark:text-white"
          type="button"
          @click="openAuthDialog('login')"
        >
          Se connecter
        </button>
      </template>
    </div>
  </header>
</template>
