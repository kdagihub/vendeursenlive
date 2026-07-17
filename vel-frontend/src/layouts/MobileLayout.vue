<script setup lang="ts">
import BottomMenu from '@/components/BottomMenu.vue'
import MobileHeader from '@/components/MobileHeader.vue'
</script>

<template>
  <div class="min-h-screen bg-[#f7fafc] text-slate-950 lg:hidden dark:bg-[#0d0d0e] dark:text-white">
    <MobileHeader />
    <main class="min-h-screen pt-15 pb-20">
      <slot />
    </main>
    <BottomMenu />
  </div>
</template>
