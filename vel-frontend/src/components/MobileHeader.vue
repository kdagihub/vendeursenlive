<script setup lang="ts">
import { Search, UserRound } from '@lucide/vue'

import { useAuthStore } from '@/stores/auth'
import BrandMark from '@/components/BrandMark.vue'

const auth = useAuthStore()
</script>

<template>
  <header
    class="fixed inset-x-0 top-0 z-40 flex h-15 items-center justify-between bg-[#f7fafc]/88 px-4 backdrop-blur lg:hidden dark:bg-[#0d0d0e]/88"
  >
    <BrandMark />
    <div class="flex items-center gap-1">
      <a
        href="#mobile-search"
        class="grid h-10 w-10 place-items-center rounded-full text-slate-800 no-underline hover:bg-slate-100 dark:text-white dark:hover:bg-white/10"
        aria-label="Rechercher"
      >
        <Search :size="22" />
      </a>
      <RouterLink
        :to="auth.isAuthenticated ? '/app' : '/login'"
        class="grid h-10 w-10 place-items-center rounded-full text-slate-800 no-underline hover:bg-slate-100 dark:text-white dark:hover:bg-white/10"
        aria-label="Compte"
      >
        <UserRound :size="22" />
      </RouterLink>
    </div>
  </header>
</template>
