<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'

import DesktopLayout from '@/layouts/DesktopLayout.vue'
import MobileLayout from '@/layouts/MobileLayout.vue'
import AuthDialog from '@/components/auth/AuthDialog.vue'
import SellerLiveDialog from '@/components/SellerLiveDialog.vue'

const mediaQuery = window.matchMedia('(min-width: 1024px)')
const isDesktop = ref(mediaQuery.matches)

function syncLayout(event?: MediaQueryListEvent) {
  isDesktop.value = event?.matches ?? mediaQuery.matches
}

onMounted(() => {
  mediaQuery.addEventListener('change', syncLayout)
})

onBeforeUnmount(() => mediaQuery.removeEventListener('change', syncLayout))
</script>

<template>
  <DesktopLayout v-if="isDesktop"><slot /></DesktopLayout>
  <MobileLayout v-else><slot /></MobileLayout>
  <AuthDialog />
  <SellerLiveDialog />
</template>
