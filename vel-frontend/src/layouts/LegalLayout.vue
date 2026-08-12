<script setup lang="ts">
import { ArrowLeft } from '@lucide/vue'

import BottomMenu from '@/components/BottomMenu.vue'
import BrandMark from '@/components/BrandMark.vue'
import LegalFooter from '@/components/LegalFooter.vue'
</script>

<template>
  <div
    class="flex min-h-screen flex-col bg-[#f7fafc] text-slate-950 dark:bg-[#0d0d0e] dark:text-white"
  >
    <header
      class="sticky top-0 z-40 bg-[#f7fafc]/94 px-4 backdrop-blur sm:px-6 lg:px-8 dark:bg-[#0d0d0e]/94"
    >
      <div
        class="mx-auto flex min-h-17 w-full max-w-[1500px] flex-wrap items-center justify-between gap-3 py-2"
      >
        <BrandMark />

        <nav
          class="flex items-center gap-3 text-xs font-bold sm:gap-5 sm:text-sm"
          aria-label="Pages légales"
        >
          <RouterLink
            class="text-slate-600 hover:text-[#d92035] dark:text-slate-300 dark:hover:text-[#ff7381]"
            to="/terms"
          >
            CGU
          </RouterLink>
          <RouterLink
            class="text-slate-600 hover:text-[#d92035] dark:text-slate-300 dark:hover:text-[#ff7381]"
            to="/privacy"
          >
            Confidentialité
          </RouterLink>
          <RouterLink
            class="inline-flex h-9 items-center gap-2 rounded-md bg-[#17121f] px-3 text-white no-underline hover:bg-black dark:bg-white dark:text-[#17121f] dark:hover:bg-slate-200"
            to="/"
          >
            <ArrowLeft :size="17" />
            <span class="hidden sm:inline">Retour à l’accueil</span>
            <span class="sm:hidden">Accueil</span>
          </RouterLink>
        </nav>
      </div>
    </header>

    <main class="flex-1">
      <slot />
    </main>

    <LegalFooter mobile-nav-offset />
    <BottomMenu />
  </div>
</template>
