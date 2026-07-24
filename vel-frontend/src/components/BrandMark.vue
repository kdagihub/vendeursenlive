<script setup lang="ts">
import logo from '@/assets/img/logo_vel_soft.png'

defineProps<{
  compact?: boolean
}>()
</script>

<template>
  <RouterLink
    class="inline-flex min-w-0 items-center gap-2.5 text-[#17121f] no-underline dark:text-white"
    to="/"
  >
    <img
      class="h-10 w-10 shrink-0 rounded-full object-contain sm:h-11 sm:w-11"
      :src="logo"
      alt="VendeursEnLive"
    />
    <span v-if="!compact" class="truncate text-lg font-black sm:text-xl">
      VendeursEn<span class="text-[#ff3447]">Live</span>
    </span>
  </RouterLink>
</template>
