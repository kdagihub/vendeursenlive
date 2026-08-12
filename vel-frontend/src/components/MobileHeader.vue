<script setup lang="ts">
import { Radio, Search, UserRound } from '@lucide/vue'

import AccountMenu from '@/components/AccountMenu.vue'
import { useAuthStore } from '@/stores/auth'
import { useAuthDialog } from '@/composables/useAuthDialog'
import { useSellerLiveDialog } from '@/composables/useSellerLiveDialog'
import BrandMark from '@/components/BrandMark.vue'

const auth = useAuthStore()
const { openAuthDialog } = useAuthDialog()
const { openLiveDialog } = useSellerLiveDialog()
</script>

<template>
  <header
    class="fixed inset-x-0 top-0 z-40 flex h-20 flex-col bg-[#f7fafc]/88 px-4 backdrop-blur lg:hidden dark:bg-[#0d0d0e]/88"
  >
    <div class="flex h-14 w-full items-center justify-between">
      <BrandMark />
      <div class="flex items-center gap-1">
        <button
          v-if="auth.user?.is_seller"
          class="grid h-10 w-10 place-items-center rounded-full text-[#d92035] hover:bg-red-50 dark:text-[#ff6675] dark:hover:bg-white/10"
          type="button"
          aria-label="Connecter mon LIVE"
          @click="openLiveDialog"
        >
          <Radio :size="22" />
        </button>
        <RouterLink
          to="/explore"
          class="grid h-10 w-10 place-items-center rounded-full text-slate-800 no-underline hover:bg-slate-100 dark:text-white dark:hover:bg-white/10"
          aria-label="Rechercher"
        >
          <Search :size="22" />
        </RouterLink>
        <AccountMenu v-if="auth.isAuthenticated" compact />
        <button
          v-else
          class="grid h-10 w-10 place-items-center rounded-full text-slate-800 hover:bg-slate-100 dark:text-white dark:hover:bg-white/10"
          type="button"
          aria-label="Se connecter"
          @click="openAuthDialog('login')"
        >
          <UserRound :size="22" />
        </button>
      </div>
    </div>

    <nav
      class="flex h-6 w-full items-start justify-end gap-3 text-[10px] font-bold"
      aria-label="Informations légales"
    >
      <RouterLink class="text-slate-500 no-underline dark:text-slate-400" to="/terms">
        CGU
      </RouterLink>
      <RouterLink class="text-slate-500 no-underline dark:text-slate-400" to="/privacy">
        Confidentialité
      </RouterLink>
    </nav>
  </header>
</template>
