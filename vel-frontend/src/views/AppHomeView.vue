<script setup lang="ts">
import {
  BadgeCheck,
  CalendarDays,
  CircleAlert,
  KeyRound,
  LogOut,
  Mail,
  Phone,
  RefreshCw,
  ShieldCheck,
  Store,
  UserRound,
} from '@lucide/vue'
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import ResponsiveLayout from '@/layouts/ResponsiveLayout.vue'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

const passwordForm = reactive({
  currentPassword: '',
  newPassword: '',
})
const profileForm = reactive({
  fullName: '',
  shopName: '',
})
const successMessage = ref<string | null>(null)

const fullNameChanged = computed(() => profileForm.fullName.trim() !== (auth.user?.full_name ?? ''))
const shopNameChanged = computed(() => profileForm.shopName.trim() !== (auth.user?.shop_name ?? ''))
const canSaveProfile = computed(
  () =>
    (fullNameChanged.value && profileForm.fullName.trim().length > 0) ||
    (auth.user?.is_seller === true &&
      shopNameChanged.value &&
      profileForm.shopName.trim().length > 0),
)
const avatarInitials = computed(() => {
  const source = auth.user?.full_name || auth.user?.shop_name || auth.accountLabel
  return source
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase())
    .join('')
})
const memberSince = computed(() => {
  if (!auth.user?.member_since) return 'Date indisponible'
  return new Intl.DateTimeFormat('fr-CI', {
    month: 'long',
    year: 'numeric',
  }).format(new Date(auth.user.member_since))
})
const authMethodLabels = computed(() => {
  const labels: Record<string, string> = {
    email: 'Email',
    google: 'Google',
    phone: 'Téléphone',
    tiktok: 'TikTok',
  }
  return (auth.user?.auth_methods ?? []).map((method) => labels[method] ?? method).join(', ')
})
const profileProgress = computed(() => {
  const required = auth.user?.is_seller ? 2 : 1
  const completed = Number(Boolean(auth.user?.full_name)) + Number(Boolean(auth.user?.shop_name))
  return Math.min(100, Math.round((completed / required) * 100))
})

watch(
  () => auth.user,
  (user) => {
    profileForm.fullName = user?.full_name ?? ''
    profileForm.shopName = user?.shop_name ?? ''
  },
  { immediate: true },
)

onMounted(async () => {
  if (!auth.user?.member_since) await auth.fetchMe()
})

async function refreshSession() {
  try {
    await auth.refresh()
    successMessage.value = 'Tes informations sont à jour.'
  } catch {
    successMessage.value = null
  }
}

async function logout() {
  await auth.logout()
  await router.push('/')
}

async function changePassword() {
  try {
    await auth.changePassword(passwordForm.currentPassword, passwordForm.newPassword)
    passwordForm.currentPassword = ''
    passwordForm.newPassword = ''
    successMessage.value = 'Mot de passe mis à jour.'
  } catch {
    successMessage.value = null
  }
}

async function updateProfile() {
  try {
    await auth.updateProfile(
      fullNameChanged.value ? profileForm.fullName.trim() : undefined,
      auth.user?.is_seller && shopNameChanged.value ? profileForm.shopName.trim() : undefined,
    )
    successMessage.value = 'Profil mis à jour.'
    if (route.query.onboarding === '1') await router.replace('/profile')
  } catch {
    successMessage.value = null
  }
}
</script>

<template>
  <ResponsiveLayout>
    <section class="workspace-view">
      <div class="workspace-header">
        <div>
          <p class="eyebrow">Mon profil</p>
          <h1>Bonjour {{ auth.displayName }}</h1>
          <p class="workspace-intro">Gère ton profil et les accès liés à ton compte.</p>
        </div>

        <div class="action-row">
          <Button outlined :loading="auth.loading" type="button" @click="refreshSession">
            <RefreshCw :size="17" aria-hidden="true" />
            Actualiser
          </Button>
          <Button severity="danger" :loading="auth.loading" type="button" @click="logout">
            <LogOut :size="17" aria-hidden="true" />
            Déconnexion
          </Button>
        </div>
      </div>

      <Message v-if="successMessage" severity="success">{{ successMessage }}</Message>
      <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>
      <Message v-if="route.query.onboarding === '1'" severity="info">
        Ton compte est créé. Complète maintenant ton profil, ou reviens-y plus tard.
      </Message>

      <section class="profile-summary" aria-labelledby="profile-name">
        <img
          v-if="auth.user?.avatar_url"
          class="profile-avatar"
          :src="auth.user.avatar_url"
          alt="Photo du profil"
          referrerpolicy="no-referrer"
        />
        <span v-else class="profile-avatar profile-avatar--fallback" aria-hidden="true">
          {{ avatarInitials }}
        </span>

        <div class="profile-summary-main">
          <div class="profile-name-row">
            <h2 id="profile-name">{{ auth.displayName }}</h2>
            <span v-if="auth.user?.account_verified" class="verified-pill">
              <BadgeCheck :size="16" aria-hidden="true" /> Vérifié
            </span>
            <span v-else class="pending-pill">
              <CircleAlert :size="16" aria-hidden="true" /> À vérifier
            </span>
          </div>
          <p>
            {{ auth.accountLabel
            }}<template v-if="auth.user?.shop_name"> · {{ auth.user.shop_name }}</template>
          </p>
          <span class="member-since">
            <CalendarDays :size="15" aria-hidden="true" /> Membre depuis {{ memberSince }}
          </span>
        </div>

        <div class="profile-completion" aria-label="Complétion du profil">
          <div>
            <span>Profil complété</span><strong>{{ profileProgress }} %</strong>
          </div>
          <span class="profile-progress-track"
            ><span :style="{ width: `${profileProgress}%` }"
          /></span>
        </div>
      </section>

      <div class="workspace-grid">
        <Card>
          <template #title>
            <span class="profile-card-title">
              <Store v-if="auth.user?.is_seller" :size="20" aria-hidden="true" />
              <UserRound v-else :size="20" aria-hidden="true" />
              Informations du profil
            </span>
          </template>
          <template #content>
            <form class="auth-form" @submit.prevent="updateProfile">
              <label class="field">
                <span>Nom complet <small>(facultatif)</small></span>
                <InputText
                  v-model="profileForm.fullName"
                  autocomplete="name"
                  maxlength="255"
                  placeholder="Awa Kouamé"
                />
              </label>

              <label v-if="auth.user?.is_seller" class="field">
                <span>Nom de la boutique</span>
                <InputText
                  v-model="profileForm.shopName"
                  maxlength="255"
                  placeholder="Boutique Awa"
                  :required="route.query.onboarding === '1'"
                />
              </label>

              <Button
                class="full-control"
                :disabled="!canSaveProfile"
                label="Enregistrer les modifications"
                :loading="auth.loading"
                type="submit"
              />
            </form>
          </template>
        </Card>

        <Card>
          <template #title>
            <span class="profile-card-title"
              ><ShieldCheck :size="20" aria-hidden="true" />Compte et sécurité</span
            >
          </template>
          <template #content>
            <dl class="profile-details">
              <div v-if="auth.user?.phone_number">
                <dt><Phone :size="17" aria-hidden="true" />Téléphone</dt>
                <dd>{{ auth.user.phone_number }}</dd>
              </div>
              <div v-if="auth.user?.email">
                <dt><Mail :size="17" aria-hidden="true" />Email</dt>
                <dd>{{ auth.user.email }}</dd>
              </div>
              <div>
                <dt><KeyRound :size="17" aria-hidden="true" />Connexion</dt>
                <dd>{{ authMethodLabels || 'Méthode sécurisée' }}</dd>
              </div>
              <div>
                <dt><ShieldCheck :size="17" aria-hidden="true" />Statut</dt>
                <dd>
                  {{ auth.user?.account_verified ? 'Compte vérifié' : 'Vérification requise' }}
                </dd>
              </div>
            </dl>
          </template>
        </Card>

        <Card v-if="auth.user?.can_change_password" class="security-card">
          <template #title>
            <span class="profile-card-title"
              ><KeyRound :size="20" aria-hidden="true" />Changer le mot de passe</span
            >
          </template>
          <template #content>
            <form class="auth-form" @submit.prevent="changePassword">
              <label class="field">
                <span>Mot de passe actuel</span>
                <Password
                  v-model="passwordForm.currentPassword"
                  :feedback="false"
                  autocomplete="current-password"
                  input-id="current-password"
                  required
                  toggle-mask
                />
              </label>
              <label class="field">
                <span>Nouveau mot de passe</span>
                <Password
                  v-model="passwordForm.newPassword"
                  autocomplete="new-password"
                  input-id="new-password"
                  required
                  toggle-mask
                />
              </label>
              <Button
                class="full-control"
                label="Mettre à jour le mot de passe"
                :loading="auth.loading"
                type="submit"
              />
            </form>
          </template>
        </Card>
      </div>
    </section>
  </ResponsiveLayout>
</template>
