<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'
import LiveRoomContent from '@/components/LiveRoomContent.vue'
import ResponsiveLayout from '@/layouts/ResponsiveLayout.vue'

const route = useRoute()
const discovery = useLiveDiscoveryStore()
const live = computed(() => discovery.findLive(String(route.params.id)))
</script>

<template>
  <ResponsiveLayout>
    <LiveRoomContent v-if="live" :live="live" />
    <section v-else class="mx-auto max-w-xl px-5 py-20 text-center">
      <h1 class="text-2xl font-black">Ce live n'est plus disponible</h1>
      <p class="mt-2 text-slate-500">Il est peut-être terminé ou a été retiré par le vendeur.</p>
      <RouterLink
        class="mt-6 inline-flex h-11 items-center rounded-md bg-[#ff3447] px-5 font-bold text-white no-underline"
        to="/"
      >
        Voir les lives en cours
      </RouterLink>
    </section>
  </ResponsiveLayout>
</template>
