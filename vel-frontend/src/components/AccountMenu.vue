<script setup lang="ts">
import { ChevronDown, LogOut, UserRound } from '@lucide/vue'
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

defineProps<{
  compact?: boolean
}>()

const auth = useAuthStore()
const route = useRoute()
const router = useRouter()
const isOpen = ref(false)
const root = ref<HTMLElement | null>(null)

const initials = computed(() => {
  const source = auth.user?.full_name || auth.user?.shop_name || auth.accountLabel
  return source
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase())
    .join('')
})

watch(
  () => route.fullPath,
  () => {
    isOpen.value = false
  },
)

function closeOnOutsideClick(event: PointerEvent) {
  if (!root.value?.contains(event.target as Node)) isOpen.value = false
}

function closeOnEscape(event: KeyboardEvent) {
  if (event.key === 'Escape') isOpen.value = false
}

async function logout() {
  isOpen.value = false
  await auth.logout()
  await router.push('/')
}

onMounted(() => {
  document.addEventListener('pointerdown', closeOnOutsideClick)
  document.addEventListener('keydown', closeOnEscape)
})

onBeforeUnmount(() => {
  document.removeEventListener('pointerdown', closeOnOutsideClick)
  document.removeEventListener('keydown', closeOnEscape)
})
</script>

<template>
  <div ref="root" class="relative">
    <button
      class="account-menu-trigger inline-flex h-10 items-center justify-center gap-2 rounded-md px-1.5 text-sm font-bold text-slate-800 hover:bg-slate-100 dark:text-white dark:hover:bg-white/10"
      type="button"
      aria-haspopup="menu"
      :aria-expanded="isOpen"
      aria-label="Ouvrir le menu du compte"
      @click="isOpen = !isOpen"
    >
      <span
        class="grid h-8 w-8 shrink-0 place-items-center overflow-hidden rounded-full bg-[#211536] text-xs font-black text-white ring-2 ring-white dark:ring-[#36363c]"
      >
        <img
          v-if="auth.user?.avatar_url"
          :src="auth.user.avatar_url"
          alt=""
          class="h-full w-full object-cover"
        />
        <span v-else>{{ initials }}</span>
      </span>
      <span v-if="!compact" class="max-w-24 truncate">Mon profil</span>
      <ChevronDown
        v-if="!compact"
        :size="16"
        aria-hidden="true"
        class="transition-transform"
        :class="{ 'rotate-180': isOpen }"
      />
    </button>

    <div
      v-if="isOpen"
      class="absolute top-[calc(100%+0.5rem)] right-0 z-50 w-64 overflow-hidden rounded-md border border-slate-200 bg-white py-2 text-slate-900 shadow-xl dark:border-white/10 dark:bg-[#202024] dark:text-white"
      role="menu"
      aria-label="Menu du compte"
    >
      <div class="border-b border-slate-100 px-4 pt-1 pb-3 dark:border-white/10">
        <p class="truncate text-sm font-extrabold">{{ auth.displayName }}</p>
        <p class="mt-0.5 text-xs text-slate-500 dark:text-slate-400">{{ auth.accountLabel }}</p>
      </div>

      <RouterLink
        class="flex min-h-11 items-center gap-3 px-4 text-sm font-semibold text-slate-800 no-underline hover:bg-slate-100 dark:text-white dark:hover:bg-white/8"
        to="/profile"
        role="menuitem"
      >
        <UserRound :size="19" aria-hidden="true" />
        Voir le profil
      </RouterLink>
      <button
        class="flex min-h-11 w-full items-center gap-3 px-4 text-left text-sm font-semibold text-[#d92035] hover:bg-red-50 dark:text-[#ff6675] dark:hover:bg-white/8"
        type="button"
        role="menuitem"
        :disabled="auth.loading"
        @click="logout"
      >
        <LogOut :size="19" aria-hidden="true" />
        Déconnexion
      </button>
    </div>
  </div>
</template>
