<script setup lang="ts">
import { Maximize, Pause, Play, Volume2, VolumeX } from '@lucide/vue'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'

import poster from '@/assets/img/couverture.png'

const props = defineProps<{
  username: string
  previewVideoUrl: string
}>()

const iframe = ref<HTMLIFrameElement | null>(null)
const previewVideo = ref<HTMLVideoElement | null>(null)
const isReady = ref(false)
const isPlaying = ref(true)
const isMuted = ref(true)
const embedEnabled = import.meta.env.VITE_TIKTOK_LIVE_EMBED_ENABLED === 'true'

const embedUrl = computed(() => {
  const username = props.username.trim().replace(/^@/, '')
  const url = new URL(`https://www.tiktok.com/embed/live/@${encodeURIComponent(username)}`)
  url.searchParams.set('autoplay', '1')
  url.searchParams.set('muted', '1')
  url.searchParams.set('controls', '1')
  url.searchParams.set('embed_domain', window.location.hostname)
  return url.toString()
})

function sendPlayerMessage(type: 'play' | 'pause' | 'mute' | 'unMute') {
  iframe.value?.contentWindow?.postMessage(
    { type, 'x-tiktok-player': true },
    'https://www.tiktok.com',
  )
}

async function togglePlayback() {
  if (embedEnabled) {
    sendPlayerMessage(isPlaying.value ? 'pause' : 'play')
    return
  }

  if (!previewVideo.value) return
  if (previewVideo.value.paused) await previewVideo.value.play()
  else previewVideo.value.pause()
  isPlaying.value = !previewVideo.value.paused
}

function toggleMute() {
  if (embedEnabled) {
    sendPlayerMessage(isMuted.value ? 'unMute' : 'mute')
  } else if (previewVideo.value) {
    previewVideo.value.muted = !previewVideo.value.muted
    isMuted.value = previewVideo.value.muted
  }
}

function requestFullscreen() {
  void (iframe.value ?? previewVideo.value)?.requestFullscreen()
}

function receivePlayerMessage(event: MessageEvent) {
  if (event.origin !== 'https://www.tiktok.com') return
  const message = event.data as { 'x-tiktok-player'?: boolean; type?: string; value?: unknown }
  if (message['x-tiktok-player'] !== true) return

  if (message.type === 'onPlayerReady') isReady.value = true
  if (message.type === 'onStateChange') isPlaying.value = message.value === 1
  if (message.type === 'onMute') isMuted.value = message.value === true
}

onMounted(() => window.addEventListener('message', receivePlayerMessage))
onBeforeUnmount(() => window.removeEventListener('message', receivePlayerMessage))
</script>

<template>
  <div
    class="relative aspect-[9/16] max-h-[78vh] w-full overflow-hidden bg-black lg:aspect-video lg:max-h-none"
  >
    <iframe
      v-if="embedEnabled"
      ref="iframe"
      class="h-full w-full border-0"
      :src="embedUrl"
      allow="autoplay; fullscreen"
      allowfullscreen
      loading="lazy"
      title="TikTok LIVE"
    />
    <video
      v-else
      ref="previewVideo"
      class="h-full w-full object-cover"
      :poster="poster"
      :src="previewVideoUrl"
      autoplay
      loop
      muted
      playsinline
    />

    <div
      v-if="!embedEnabled"
      class="absolute inset-x-0 top-0 bg-black/65 px-4 py-2.5 text-xs text-white/80"
    >
      Prévisualisation locale — activation TikTok après autorisation du domaine
    </div>

    <div
      class="absolute right-3 bottom-3 flex items-center gap-1.5 rounded-md bg-black/65 p-1.5 text-white backdrop-blur-sm"
    >
      <button
        class="grid h-9 w-9 place-items-center rounded hover:bg-white/15 disabled:opacity-50"
        type="button"
        :disabled="embedEnabled && !isReady"
        :aria-label="isPlaying ? 'Mettre en pause' : 'Lire'"
        @click="togglePlayback"
      >
        <Pause v-if="isPlaying" :size="19" />
        <Play v-else :size="19" />
      </button>
      <button
        class="grid h-9 w-9 place-items-center rounded hover:bg-white/15"
        type="button"
        :aria-label="isMuted ? 'Activer le son' : 'Couper le son'"
        @click="toggleMute"
      >
        <VolumeX v-if="isMuted" :size="19" />
        <Volume2 v-else :size="19" />
      </button>
      <button
        class="grid h-9 w-9 place-items-center rounded hover:bg-white/15"
        type="button"
        aria-label="Plein écran"
        @click="requestFullscreen"
      >
        <Maximize :size="19" />
      </button>
    </div>
  </div>
</template>
