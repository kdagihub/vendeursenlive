<script setup lang="ts">
import { ArrowLeft, Check, ChevronDown, ChevronUp, MapPin, ShoppingBag, Users } from '@lucide/vue'
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'
import { useLiveDiscoveryStore } from '@/stores/liveDiscovery'
import TikTokLivePlayer from '@/components/TikTokLivePlayer.vue'
import type { LivePreview } from '@/types/live'

const props = defineProps<{
  live: LivePreview
}>()

const auth = useAuthStore()
const discovery = useLiveDiscoveryStore()
const router = useRouter()
const verificationDialogVisible = ref(false)
let navigationLocked = false
let navigationTimer: ReturnType<typeof setTimeout> | undefined
let touchStartX = 0
let touchStartY = 0

const formattedPrice = computed(
  () => `${new Intl.NumberFormat('fr-FR').format(props.live.product.price)} FCFA`,
)

async function startOrder() {
  if (!auth.isAuthenticated) {
    await router.push({
      name: 'live-room',
      params: { id: props.live.id },
      query: { auth: 'login', redirect: `/live/${props.live.id}` },
    })
    return
  }

  if (!auth.user?.account_verified) {
    verificationDialogVisible.value = true
    return
  }
}

async function resendVerification() {
  await auth.requestEmailVerification()
}

async function navigateLive(direction: 'previous' | 'next') {
  if (navigationLocked) return
  const target = discovery.adjacentLive(props.live.id, direction)
  if (!target) return

  navigationLocked = true
  await router.replace({ name: 'live-room', params: { id: target.id } })
  navigationTimer = setTimeout(() => {
    navigationLocked = false
  }, 550)
}

function handleWheel(event: WheelEvent) {
  if (Math.abs(event.deltaY) < 35) return
  void navigateLive(event.deltaY > 0 ? 'next' : 'previous')
}

function handleTouchStart(event: TouchEvent) {
  const touch = event.touches[0]
  if (!touch) return
  touchStartX = touch.clientX
  touchStartY = touch.clientY
}

function handleTouchEnd(event: TouchEvent) {
  const touch = event.changedTouches[0]
  if (!touch) return
  const deltaX = touch.clientX - touchStartX
  const deltaY = touch.clientY - touchStartY

  if (Math.abs(deltaY) < 60 || Math.abs(deltaY) <= Math.abs(deltaX)) return
  void navigateLive(deltaY < 0 ? 'next' : 'previous')
}

function handleKeyboard(event: KeyboardEvent) {
  const element = event.target as HTMLElement | null
  if (element?.matches('input, textarea, select, button')) return
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    void navigateLive('next')
  }
  if (event.key === 'ArrowUp') {
    event.preventDefault()
    void navigateLive('previous')
  }
}

onMounted(() => window.addEventListener('keydown', handleKeyboard))
onBeforeUnmount(() => {
  window.removeEventListener('keydown', handleKeyboard)
  if (navigationTimer) clearTimeout(navigationTimer)
})
</script>

<template>
  <div class="mx-auto w-full max-w-[1420px] lg:p-8">
    <RouterLink
      to="/"
      class="mx-4 my-3 inline-flex h-10 items-center gap-2 text-sm font-bold text-slate-700 no-underline lg:mx-0 lg:mt-0 dark:text-slate-200"
    >
      <ArrowLeft :size="18" />
      Retour aux lives
    </RouterLink>

    <div class="grid items-start lg:grid-cols-[minmax(0,1.35fr)_minmax(360px,0.65fr)] lg:gap-6">
      <div
        class="relative touch-pan-x"
        @touchend="handleTouchEnd"
        @touchstart="handleTouchStart"
        @wheel.prevent="handleWheel"
      >
        <TikTokLivePlayer
          :key="live.id"
          :preview-video-url="live.videoUrl"
          :username="live.tiktokUsername"
        />

        <div
          class="absolute top-1/2 right-3 z-20 flex -translate-y-1/2 flex-col gap-2"
          aria-label="Changer de live"
        >
          <button
            class="grid h-11 w-11 place-items-center rounded-full bg-black/70 text-white shadow-lg backdrop-blur transition hover:bg-black focus-visible:ring-2 focus-visible:ring-[#ff3447] focus-visible:outline-none dark:bg-white/14 dark:hover:bg-white/22"
            title="Live précédent"
            type="button"
            aria-label="Voir le live précédent"
            @click.stop="navigateLive('previous')"
          >
            <ChevronUp :size="23" />
          </button>
          <button
            class="grid h-11 w-11 place-items-center rounded-full bg-black/70 text-white shadow-lg backdrop-blur transition hover:bg-black focus-visible:ring-2 focus-visible:ring-[#ff3447] focus-visible:outline-none dark:bg-white/14 dark:hover:bg-white/22"
            title="Live suivant"
            type="button"
            aria-label="Voir le live suivant"
            @click.stop="navigateLive('next')"
          >
            <ChevronDown :size="23" />
          </button>
        </div>
      </div>

      <section
        class="bg-white px-4 py-5 lg:rounded-md lg:border lg:border-slate-200 lg:p-6 dark:bg-[#151516] dark:lg:border-white/10"
      >
        <div class="flex items-start justify-between gap-4">
          <div>
            <span
              class="inline-flex items-center gap-1.5 text-xs font-black text-[#d92035] uppercase"
            >
              <span class="h-2 w-2 animate-pulse rounded-full bg-[#ff3447]" />
              En direct
            </span>
            <h1 class="mt-2 text-2xl leading-tight font-black text-[#211536] dark:text-white">
              {{ live.title }}
            </h1>
            <p class="mt-2 font-bold">
              {{ live.sellerName }}
              <span class="font-normal text-slate-500">{{ live.sellerHandle }}</span>
            </p>
          </div>
        </div>

        <div class="mt-4 flex flex-wrap gap-4 text-sm text-slate-500 dark:text-slate-400">
          <span class="inline-flex items-center gap-1.5"
            ><Users :size="17" /> {{ live.viewerCount }} spectateurs</span
          >
          <span class="inline-flex items-center gap-1.5"
            ><MapPin :size="17" /> {{ live.location }}</span
          >
        </div>

        <div class="mt-6 border-y border-slate-200 py-5 dark:border-white/10">
          <p class="text-xs font-black text-slate-500 uppercase">Produit présenté</p>
          <div class="mt-2 flex items-end justify-between gap-4">
            <div>
              <h2 class="text-xl font-black">{{ live.product.name }}</h2>
              <p class="mt-1 text-sm text-slate-500">
                Commande soumise au vendeur pour validation.
              </p>
            </div>
            <p class="shrink-0 text-xl font-black text-[#d92035]">{{ formattedPrice }}</p>
          </div>
        </div>

        <button
          class="mt-6 inline-flex h-13 w-full items-center justify-center gap-2 rounded-md bg-[#ff3447] px-5 text-base font-black text-white hover:bg-[#d92035]"
          type="button"
          @click="startOrder"
        >
          <ShoppingBag :size="21" />
          Commander maintenant
        </button>

        <ul class="mt-5 space-y-2 text-sm text-slate-600 dark:text-slate-300">
          <li class="flex items-center gap-2">
            <Check :size="17" class="text-emerald-600" /> Aucun stock bloqué
          </li>
          <li class="flex items-center gap-2">
            <Check :size="17" class="text-emerald-600" /> Validation manuelle par le vendeur
          </li>
        </ul>
      </section>
    </div>
  </div>

  <Dialog
    v-model:visible="verificationDialogVisible"
    :draggable="false"
    header="Vérifie ton email pour commander"
    modal
    class="verification-dialog"
  >
    <p class="m-0 text-sm leading-6 text-slate-600">
      Ton compte reste accessible en lecture. La vérification protège les commandes envoyées aux
      vendeurs.
    </p>
    <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>
    <template #footer>
      <Button
        label="Plus tard"
        severity="secondary"
        text
        @click="verificationDialogVisible = false"
      />
      <Button label="Renvoyer le lien" :loading="auth.loading" @click="resendVerification" />
    </template>
  </Dialog>
</template>
