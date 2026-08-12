<script setup lang="ts">
import {
  BadgeCheck,
  CircleAlert,
  ExternalLink,
  Radio,
  Square,
  Store,
  UserRound,
  X,
} from '@lucide/vue'
import { computed, ref, watch } from 'vue'
import { useRouter } from 'vue-router'

import { useSellerLiveDialog } from '@/composables/useSellerLiveDialog'
import { useAuthStore } from '@/stores/auth'
import { useSellerLiveStore } from '@/stores/sellerLive'

const auth = useAuthStore()
const sellerLive = useSellerLiveStore()
const router = useRouter()
const { closeLiveDialog, isLiveDialogOpen } = useSellerLiveDialog()

const shopName = ref('')
const liveUrl = ref('')
const verificationSent = ref(false)
const successMessage = ref<string | null>(null)

const requiresShopName = computed(() => auth.user?.is_seller === true && !auth.user.shop_name)
const canCreateLive = computed(
  () =>
    auth.user?.is_seller === true &&
    auth.user.account_verified &&
    !requiresShopName.value &&
    !sellerLive.current,
)

watch(
  isLiveDialogOpen,
  async (open) => {
    if (!open) return

    successMessage.value = null
    verificationSent.value = false
    auth.error = null
    sellerLive.clearError()

    if (!auth.isAuthenticated) {
      const hasSession = await auth.fetchMe()
      if (!hasSession) {
        await closeLiveDialog()
        return
      }
    }

    shopName.value = auth.user?.shop_name ?? ''
    if (auth.user?.is_seller) await sellerLive.fetchCurrent()
  },
  { immediate: true },
)

async function saveShopName() {
  if (!shopName.value.trim()) return
  try {
    await auth.updateProfile(undefined, shopName.value.trim())
    successMessage.value = 'Ta boutique est prête. Tu peux maintenant connecter ton LIVE.'
  } catch {
    successMessage.value = null
  }
}

async function startLive() {
  if (!liveUrl.value.trim()) return
  try {
    await sellerLive.start(liveUrl.value.trim())
    liveUrl.value = ''
    successMessage.value = 'Ton LIVE est connecté à VendeursEnLive.'
  } catch {
    successMessage.value = null
  }
}

async function endLive() {
  try {
    await sellerLive.end()
    successMessage.value = 'Le LIVE est terminé.'
  } catch {
    successMessage.value = null
  }
}

async function requestVerification() {
  try {
    await auth.requestEmailVerification()
    verificationSent.value = true
  } catch {
    verificationSent.value = false
  }
}

async function openSellerRegistration() {
  await closeLiveDialog()
  await router.push({
    path: '/',
    query: { auth: 'register', account: 'seller' },
  })
}

function handleVisibility(visible: boolean) {
  if (!visible) void closeLiveDialog()
}
</script>

<template>
  <Dialog
    :visible="isLiveDialogOpen && auth.isAuthenticated"
    append-to="body"
    class="seller-live-dialog"
    :closable="false"
    :dismissable-mask="true"
    :draggable="false"
    modal
    @update:visible="handleVisibility"
  >
    <section class="seller-live-dialog-shell" aria-live="polite">
      <header class="seller-live-dialog-header">
        <span class="seller-live-dialog-mark" aria-hidden="true"><Radio :size="24" /></span>
        <div>
          <p>ESPACE VENDEUR</p>
          <h2>Connecter mon LIVE</h2>
        </div>
        <button type="button" aria-label="Fermer" @click="closeLiveDialog">
          <X :size="21" aria-hidden="true" />
        </button>
      </header>

      <div class="seller-live-dialog-body">
        <Message v-if="successMessage" severity="success">{{ successMessage }}</Message>
        <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>
        <Message v-if="sellerLive.error" severity="error">{{ sellerLive.error }}</Message>

        <div v-if="!auth.user?.is_seller" class="seller-live-empty-state">
          <span><UserRound :size="28" aria-hidden="true" /></span>
          <h3>Réservé aux comptes vendeurs</h3>
          <p>
            Ton compte client te permet de suivre les lives et de commander. Crée un espace vendeur
            pour connecter tes propres directs.
          </p>
          <Button type="button" @click="openSellerRegistration">Devenir vendeur en LIVE</Button>
        </div>

        <div v-else-if="requiresShopName" class="seller-live-step">
          <div class="seller-live-step-heading">
            <span><Store :size="21" aria-hidden="true" /></span>
            <div>
              <h3>Nomme d’abord ta boutique</h3>
              <p>Ce nom sera présenté aux clients pendant tes lives.</p>
            </div>
          </div>
          <form class="auth-form" @submit.prevent="saveShopName">
            <label class="field">
              <span>Nom de la boutique</span>
              <InputText
                v-model="shopName"
                autocomplete="organization"
                maxlength="255"
                placeholder="Maison Sigma"
                required
              />
            </label>
            <Button
              class="full-control"
              :disabled="!shopName.trim()"
              :loading="auth.loading"
              type="submit"
            >
              Continuer
            </Button>
          </form>
        </div>

        <div v-else-if="!auth.user?.account_verified" class="seller-live-empty-state">
          <span class="is-warning"><CircleAlert :size="28" aria-hidden="true" /></span>
          <h3>Vérifie ton compte</h3>
          <p>La vérification protège ta boutique avant la diffusion de ton premier LIVE.</p>
          <Button
            v-if="auth.user?.verification_channel === 'email'"
            :disabled="verificationSent"
            :loading="auth.loading"
            type="button"
            @click="requestVerification"
          >
            {{ verificationSent ? 'Email envoyé' : 'Renvoyer le lien de vérification' }}
          </Button>
        </div>

        <div v-else-if="sellerLive.current" class="seller-live-active-state">
          <span class="live-status-badge"><span aria-hidden="true" />En direct</span>
          <h3>@{{ sellerLive.current.tiktok_username }}</h3>
          <p>{{ auth.user.shop_name }}</p>
          <a :href="sellerLive.current.tiktok_live_url" target="_blank" rel="noopener noreferrer">
            Ouvrir la source du LIVE <ExternalLink :size="16" aria-hidden="true" />
          </a>
          <Button
            severity="danger"
            outlined
            :loading="sellerLive.loading"
            type="button"
            @click="endLive"
          >
            <Square :size="16" aria-hidden="true" /> Terminer le LIVE
          </Button>
        </div>

        <div v-else-if="canCreateLive" class="seller-live-step">
          <div class="seller-live-account-row">
            <span><Store :size="19" aria-hidden="true" /></span>
            <div>
              <strong>{{ auth.user.shop_name }}</strong
              ><small>Compte vendeur</small>
            </div>
            <BadgeCheck :size="20" aria-label="Compte vérifié" />
          </div>
          <form class="auth-form" @submit.prevent="startLive">
            <label class="field">
              <span>Lien partagé du LIVE TikTok</span>
              <InputText
                v-model="liveUrl"
                autocomplete="url"
                inputmode="url"
                placeholder="https://www.tiktok.com/@boutique/live"
                required
                type="url"
              />
            </label>
            <p class="seller-live-help">
              Dans TikTok, ouvre le partage de ton LIVE, copie le lien puis colle-le ici.
            </p>
            <Button
              class="full-control"
              :disabled="!liveUrl.trim()"
              :loading="sellerLive.loading"
              type="submit"
            >
              <Radio :size="18" aria-hidden="true" /> Connecter ce LIVE
            </Button>
          </form>
        </div>
      </div>
    </section>
  </Dialog>
</template>
