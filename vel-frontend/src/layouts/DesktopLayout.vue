<script setup lang="ts">
import DesktopSidebar from '@/components/DesktopSidebar.vue'
import DesktopTopbar from '@/components/DesktopTopbar.vue'
import LegalFooter from '@/components/LegalFooter.vue'
</script>

<template>
  <div
    class="hidden min-h-screen bg-[#f7fafc] text-slate-950 lg:block dark:bg-[#0d0d0e] dark:text-white"
  >
    <DesktopTopbar />
    <DesktopSidebar />
    <main class="min-h-screen pt-18 pl-64">
      <slot />
      <LegalFooter />
    </main>
  </div>
</template>
