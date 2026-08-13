<script setup lang="ts">
import { Compass, House, Package, Radio, UserRound } from '@lucide/vue'
import { useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()

const leadingItems = [
  { label: 'Accueil', to: '/', icon: House },
  { label: 'Explorer', to: '/explore', icon: Compass },
]
const trailingItems = [
  { label: 'Commandes', to: '/profile?tab=orders', icon: Package },
  { label: 'Profil', to: '/profile', icon: UserRound },
]

function connectLive() {
  if (auth.isAuthenticated) {
    return router.push({ path: '/', query: { live: 'connect' } })
  }
  return router.push({
    path: '/',
    query: { auth: 'register', redirect: '/?live=connect' },
  })
}

function isActive(to: string) {
  if (to === '/') return route.path === '/'
  if (to === '/explore') return route.path === '/explore'
  return (
    route.fullPath === to || (to === '/profile' && route.path === '/profile' && !route.query.tab)
  )
}
</script>

<template>
  <nav
    class="fixed inset-x-0 bottom-0 z-50 grid h-[calc(4.5rem+env(safe-area-inset-bottom))] grid-cols-5 bg-white/94 px-1 pb-[env(safe-area-inset-bottom)] shadow-[0_-8px_24px_rgba(15,23,42,0.06)] backdrop-blur lg:hidden dark:bg-[#111]/94 dark:shadow-[0_-8px_24px_rgba(0,0,0,0.28)]"
    aria-label="Navigation mobile"
  >
    <RouterLink
      v-for="item in leadingItems"
      :key="item.label"
      :to="item.to"
      class="flex min-w-0 flex-col items-center justify-center gap-1 px-0.5 text-[9px] leading-none font-bold no-underline"
      :class="isActive(item.to) ? 'text-[#ff3447]' : 'text-slate-500 dark:text-slate-400'"
    >
      <component :is="item.icon" :size="20" />
      <span class="truncate">{{ item.label }}</span>
    </RouterLink>

    <button
      class="group flex min-w-0 translate-y-0.5 flex-col items-center justify-center gap-1 text-slate-800 dark:text-white"
      type="button"
      aria-label="Connecter mon LIVE"
      @click="connectLive"
    >
      <span
        class="grid h-8 w-12 place-items-center rounded-md bg-slate-950 text-white shadow-[-3px_0_0_#25f4ee,3px_0_0_#ff3447,0_6px_14px_rgba(15,23,42,0.22)] transition-transform group-active:scale-95 dark:bg-white dark:text-slate-950"
      >
        <Radio :size="22" :stroke-width="2.5" aria-hidden="true" />
      </span>
      <span class="text-[8px] leading-none font-extrabold">LIVE</span>
    </button>

    <RouterLink
      v-for="item in trailingItems"
      :key="item.label"
      :to="item.to"
      class="flex min-w-0 flex-col items-center justify-center gap-1 px-0.5 text-[9px] leading-none font-bold no-underline"
      :class="isActive(item.to) ? 'text-[#ff3447]' : 'text-slate-500 dark:text-slate-400'"
    >
      <component :is="item.icon" :size="20" />
      <span class="truncate">{{ item.label }}</span>
    </RouterLink>
  </nav>
</template>
