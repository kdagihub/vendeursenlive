<script setup lang="ts">
import { Compass, House, Package, UserRound } from '@lucide/vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const items = [
  { label: 'Accueil', to: '/', icon: House },
  { label: 'Explorer', to: '/?focus=explore', icon: Compass },
  { label: 'Commandes', to: '/app?tab=orders', icon: Package },
  { label: 'Profil', to: '/app', icon: UserRound },
]

function isActive(to: string) {
  if (to === '/') return route.path === '/' && route.query.focus !== 'explore'
  if (to.startsWith('/?')) return route.path === '/' && route.query.focus === 'explore'
  return route.fullPath === to || (to === '/app' && route.path === '/app' && !route.query.tab)
}
</script>

<template>
  <nav
    class="fixed inset-x-0 bottom-0 z-50 grid h-17 grid-cols-4 bg-white/94 px-2 pb-[env(safe-area-inset-bottom)] shadow-[0_-8px_24px_rgba(15,23,42,0.06)] backdrop-blur lg:hidden dark:bg-[#111]/94 dark:shadow-[0_-8px_24px_rgba(0,0,0,0.28)]"
    aria-label="Navigation mobile"
  >
    <RouterLink
      v-for="item in items"
      :key="item.label"
      :to="item.to"
      class="flex min-w-0 flex-col items-center justify-center gap-1 text-[11px] font-bold no-underline"
      :class="isActive(item.to) ? 'text-[#ff3447]' : 'text-slate-500 dark:text-slate-400'"
    >
      <component :is="item.icon" :size="22" />
      <span class="truncate">{{ item.label }}</span>
    </RouterLink>
  </nav>
</template>
