<script setup lang="ts">
import { onMounted } from 'vue'

import DiscoveryContent from '@/components/DiscoveryContent.vue'
import ResponsiveLayout from '@/layouts/ResponsiveLayout.vue'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'

const discovery = useLiveDiscoveryStore()

onMounted(() => {
  discovery.searchQuery = ''
  discovery.activeCategory = 'all'
})
</script>

<template>
  <ResponsiveLayout>
    <DiscoveryContent />
  </ResponsiveLayout>
</template>
