<script setup lang="ts">
import { Eye, MapPin } from '@lucide/vue'
import { onBeforeUnmount, onMounted, ref } from 'vue'

import poster from '@/assets/img/couverture.png'
import type { LivePreview } from '@/types/live'

const props = defineProps<{
  live: LivePreview
  priority?: boolean
}>()

const video = ref<HTMLVideoElement | null>(null)
const isPlaying = ref(false)
let observer: IntersectionObserver | null = null

function pauseFromAnotherCard(event: Event) {
  const activeId = (event as CustomEvent<string>).detail
  if (activeId !== props.live.id) pauseVideo()
}

async function playVideo() {
  if (!video.value) return
  window.dispatchEvent(new CustomEvent('vel:preview-play', { detail: props.live.id }))
  try {
    await video.value.play()
    isPlaying.value = true
  } catch {
    isPlaying.value = false
  }
}

function pauseVideo() {
  video.value?.pause()
  isPlaying.value = false
}

onMounted(() => {
  window.addEventListener('vel:preview-play', pauseFromAnotherCard)
  observer = new IntersectionObserver(
    ([entry]) => {
      if (entry?.isIntersecting && entry.intersectionRatio >= 0.68) void playVideo()
      else pauseVideo()
    },
    { threshold: [0, 0.68, 1] },
  )
  if (video.value) observer.observe(video.value)
})

onBeforeUnmount(() => {
  observer?.disconnect()
  window.removeEventListener('vel:preview-play', pauseFromAnotherCard)
})

function formatPrice(price: number) {
  return `${new Intl.NumberFormat('fr-FR').format(price)} FCFA`
}

function formatViewers(viewerCount: number) {
  return new Intl.NumberFormat('fr-FR', { notation: 'compact' }).format(viewerCount)
}
</script>

<template>
  <RouterLink
    :to="`/live/${live.id}`"
    class="group relative block aspect-[4/5] w-58 shrink-0 overflow-hidden rounded-md bg-[#17121f] text-white no-underline sm:w-64 lg:w-60"
    :aria-label="`Regarder le live de ${live.sellerName}`"
  >
    <video
      ref="video"
      class="absolute inset-0 h-full w-full object-cover transition duration-500 group-hover:scale-[1.02]"
      :poster="poster"
      :preload="priority ? 'auto' : 'metadata'"
      :src="live.videoUrl"
      autoplay
      loop
      muted
      playsinline
      aria-hidden="true"
    />
    <div class="absolute inset-0 bg-gradient-to-t from-black via-black/10 to-black/30" />

    <div class="absolute inset-x-3 top-3 flex items-center justify-between gap-2">
      <span
        class="inline-flex h-7 items-center gap-1.5 rounded bg-[#ff3447] px-2 text-[11px] font-black uppercase"
      >
        <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-white" />
        En direct
      </span>
      <span class="inline-flex h-7 items-center gap-1 rounded bg-black/65 px-2 text-xs font-bold">
        <Eye :size="14" />
        {{ formatViewers(live.viewerCount) }}
      </span>
    </div>

    <div class="absolute inset-x-0 bottom-0 p-3.5">
      <p class="line-clamp-1 text-sm font-black">{{ live.sellerName }}</p>
      <p class="mt-0.5 flex items-center gap-1 text-xs text-white/75">
        <MapPin :size="13" />
        {{ live.location }}
      </p>
      <p class="mt-2 line-clamp-2 min-h-10 text-sm leading-5 font-bold">{{ live.title }}</p>
      <div class="mt-3 border-l-2 border-[#ff3447] bg-black/55 px-2.5 py-2 backdrop-blur-sm">
        <p class="line-clamp-1 text-xs text-white/75">{{ live.product.name }}</p>
        <p class="mt-0.5 text-sm font-black">{{ formatPrice(live.product.price) }}</p>
      </div>
    </div>
  </RouterLink>
</template>
