<script setup lang="ts">
import { ArrowLeft, Check, Mail, Smartphone, Store, UserRound, X } from '@lucide/vue'
import { computed, onBeforeUnmount, reactive, ref, watch } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { useToast } from 'primevue/usetoast'

import googleSignInIconDark from '@/assets/img/google-signin-dark.png'
import googleSignInIconLight from '@/assets/img/google-signin-light.png'
import { resolveAuthDestination, useAuthDialog } from '@/composables/useAuthDialog'
import { type AccountType, useAuthStore } from '@/stores/auth'

type AuthScreen = 'methods' | 'role' | 'credentials' | 'otp'
type AuthMethod = 'credentials' | 'google'
type CredentialMethod = 'phone' | 'email'

const auth = useAuthStore()
const route = useRoute()
const router = useRouter()
const toast = useToast()
const { closeAuthDialog, isOpen, mode, setAuthDialogMode } = useAuthDialog()

const screen = ref<AuthScreen>('methods')
const pendingMethod = ref<AuthMethod>('credentials')
const credentialMethod = ref<CredentialMethod>('phone')
const accountType = ref<AccountType | null>(null)
const challengeId = ref<string | null>(null)
const otp = ref('')
const resendSeconds = ref(0)
let resendTimer: ReturnType<typeof setInterval> | null = null

const phoneForm = reactive({ phoneNumber: '' })
const loginForm = reactive({ identifier: '', password: '' })
const emailRegisterForm = reactive({ email: '', password: '' })
const postAuthRedirectKey = 'vel_post_auth_redirect'

const dialogTitle = computed(() => {
  if (screen.value === 'role') return 'Choisis ton espace'
  if (screen.value === 'otp') return 'Entre le code reçu'
  if (screen.value === 'credentials') {
    return mode.value === 'login' ? 'Se connecter' : 'Créer un compte'
  }
  return mode.value === 'login' ? 'Connecte-toi à VendeursEnLive' : 'Inscription à VendeursEnLive'
})

const dialogSubtitle = computed(() => {
  if (screen.value === 'role') return 'Indique comment tu utiliseras VendeursEnLive.'
  if (screen.value === 'otp') return `Code envoyé au ${phoneForm.phoneNumber}.`
  if (screen.value === 'credentials') {
    return mode.value === 'login'
      ? 'Utilise ton téléphone ou ton email.'
      : 'Téléphone avec code SMS ou email avec mot de passe.'
  }
  return mode.value === 'login'
    ? 'Retrouve tes achats, tes ventes et tes lives.'
    : 'Crée ton espace client ou vendeur.'
})

watch(
  () => [isOpen.value, mode.value] as const,
  ([open]) => {
    if (open) resetFlow()
  },
)

function resetFlow() {
  screen.value = 'methods'
  pendingMethod.value = 'credentials'
  credentialMethod.value = 'phone'
  accountType.value = route.query.account === 'seller' ? 'seller' : null
  challengeId.value = null
  otp.value = ''
  phoneForm.phoneNumber = ''
  loginForm.identifier = ''
  loginForm.password = ''
  emailRegisterForm.email = ''
  emailRegisterForm.password = ''
  auth.error = null
  stopResendTimer()
}

function chooseMethod(method: AuthMethod) {
  pendingMethod.value = method
  if (mode.value === 'register' && !accountType.value) {
    screen.value = 'role'
    return
  }

  continueWithMethod()
}

function chooseAccountType(type: AccountType) {
  accountType.value = type
  continueWithMethod()
}

function continueWithMethod() {
  if (pendingMethod.value === 'google') {
    sessionStorage.setItem(postAuthRedirectKey, authDestination())
    auth.startGoogleOAuth(mode.value === 'register' ? (accountType.value ?? undefined) : undefined)
    return
  }
  screen.value = 'credentials'
}

function selectCredentialMethod(method: CredentialMethod) {
  credentialMethod.value = method
  auth.error = null
}

function goBack() {
  auth.error = null
  if (screen.value === 'otp') {
    challengeId.value = null
    otp.value = ''
    screen.value = 'credentials'
    return
  }
  if (screen.value === 'credentials' && mode.value === 'register') {
    screen.value = 'role'
    return
  }
  screen.value = 'methods'
}

async function requestPhoneOtp() {
  try {
    const challenge = await auth.requestPhoneOtp({
      phone_number: phoneForm.phoneNumber.trim(),
      purpose: mode.value,
      account_type: mode.value === 'register' ? (accountType.value ?? undefined) : undefined,
    })
    if (!challenge) return

    challengeId.value = challenge.challenge_id
    screen.value = 'otp'
    startResendCountdown(challenge.resend_after_seconds)
  } catch {
    // The store exposes a user-facing error message.
  }
}

async function verifyPhoneOtp() {
  if (!challengeId.value) return
  try {
    await auth.verifyPhoneOtp(challengeId.value, otp.value)
    await finishAuthentication()
  } catch {
    // The store exposes a user-facing error message.
  }
}

async function submitPasswordAuth() {
  try {
    if (mode.value === 'login') {
      await auth.login({
        identifier: loginForm.identifier.trim(),
        password: loginForm.password,
      })
    } else {
      if (!accountType.value) return
      await auth.register({
        email: emailRegisterForm.email.trim(),
        password: emailRegisterForm.password,
        account_type: accountType.value,
      })
    }
    await finishAuthentication()
  } catch {
    // The store exposes a user-facing error message.
  }
}

async function finishAuthentication() {
  await auth.fetchMe(false)
  sessionStorage.removeItem(postAuthRedirectKey)
  const destination = authDestination()
  await router.push(destination)
  toast.add({
    severity: 'success',
    summary: 'Connexion réussie',
    detail: `Bienvenue ${auth.displayName}. Tu es maintenant connecté.`,
    life: 5000,
  })
}

function authDestination() {
  return resolveAuthDestination(route.query.redirect)
}

function startResendCountdown(seconds: number) {
  stopResendTimer()
  resendSeconds.value = seconds
  resendTimer = setInterval(() => {
    resendSeconds.value = Math.max(0, resendSeconds.value - 1)
    if (resendSeconds.value === 0) stopResendTimer()
  }, 1000)
}

function stopResendTimer() {
  if (resendTimer) clearInterval(resendTimer)
  resendTimer = null
}

function switchMode(nextMode: 'login' | 'register') {
  void setAuthDialogMode(nextMode)
}

function handleDialogVisibility(visible: boolean) {
  if (!visible) void closeAuthDialog()
}

onBeforeUnmount(stopResendTimer)
</script>

<template>
  <Dialog
    :visible="isOpen"
    append-to="body"
    class="auth-dialog"
    :closable="false"
    :dismissable-mask="true"
    :draggable="false"
    modal
    @update:visible="handleDialogVisibility"
  >
    <section class="auth-dialog-shell" aria-live="polite">
      <header class="auth-dialog-header">
        <button
          v-if="screen !== 'methods'"
          class="auth-dialog-icon-button auth-dialog-back"
          type="button"
          aria-label="Revenir à l'étape précédente"
          @click="goBack"
        >
          <ArrowLeft :size="22" aria-hidden="true" />
        </button>
        <button
          class="auth-dialog-icon-button auth-dialog-close"
          type="button"
          aria-label="Fermer"
          @click="closeAuthDialog"
        >
          <X :size="22" aria-hidden="true" />
        </button>

        <h1>{{ dialogTitle }}</h1>
        <p>{{ dialogSubtitle }}</p>
      </header>

      <div class="auth-dialog-body">
        <Message v-if="auth.error" severity="error" class="auth-dialog-message">
          {{ auth.error }}
        </Message>

        <div v-if="screen === 'methods'" class="auth-method-list">
          <button class="auth-provider-button" type="button" @click="chooseMethod('credentials')">
            <span class="auth-provider-icon auth-provider-icon--split" aria-hidden="true">
              <Smartphone :size="20" />
              <Mail :size="18" />
            </span>
            <span>Utiliser un téléphone ou un email</span>
          </button>

          <button
            class="auth-provider-button google-provider-button"
            type="button"
            @click="chooseMethod('google')"
          >
            <span class="google-auth-icon" aria-hidden="true">
              <img class="google-auth-icon-light" :src="googleSignInIconLight" alt="" />
              <img class="google-auth-icon-dark" :src="googleSignInIconDark" alt="" />
            </span>
            <span>Continuer avec Google</span>
          </button>
        </div>

        <div v-else-if="screen === 'role'" class="auth-role-list">
          <button
            class="auth-role-button auth-role-button--customer"
            type="button"
            @click="chooseAccountType('customer')"
          >
            <UserRound :size="24" aria-hidden="true" />
            <span>
              <strong>Je suis client</strong>
              <small>Découvrir les lives et passer des commandes</small>
            </span>
          </button>
          <button
            class="auth-role-button auth-role-button--seller"
            type="button"
            @click="chooseAccountType('seller')"
          >
            <Store :size="24" aria-hidden="true" />
            <span>
              <strong>Je suis vendeur</strong>
              <small>Présenter mes produits et gérer mes ventes</small>
            </span>
          </button>
        </div>

        <template v-else-if="screen === 'credentials'">
          <div class="auth-credential-tabs" role="tablist" aria-label="Mode de connexion">
            <button
              :class="{ 'is-active': credentialMethod === 'phone' }"
              type="button"
              role="tab"
              :aria-selected="credentialMethod === 'phone'"
              @click="selectCredentialMethod('phone')"
            >
              Téléphone
            </button>
            <button
              :class="{ 'is-active': credentialMethod === 'email' }"
              type="button"
              role="tab"
              :aria-selected="credentialMethod === 'email'"
              @click="selectCredentialMethod('email')"
            >
              Email
            </button>
          </div>

          <form
            v-if="credentialMethod === 'phone'"
            class="auth-dialog-form"
            @submit.prevent="requestPhoneOtp"
          >
            <label class="field">
              <span>Numéro de téléphone</span>
              <InputText
                v-model="phoneForm.phoneNumber"
                autocomplete="tel"
                inputmode="tel"
                placeholder="+225 07 00 00 00 00"
                required
              />
            </label>
            <Button class="full-control phone-button" :loading="auth.loading" type="submit">
              <span class="phone-button-content">
                <Smartphone :size="20" aria-hidden="true" />
                <span>{{
                  mode === 'login' ? 'Recevoir mon code' : 'Créer avec mon téléphone'
                }}</span>
              </span>
            </Button>
          </form>

          <form v-else class="auth-dialog-form" @submit.prevent="submitPasswordAuth">
            <label class="field">
              <span>Email</span>
              <InputText
                v-if="mode === 'login'"
                v-model="loginForm.identifier"
                autocomplete="username"
                placeholder="email@example.com"
                required
                type="email"
              />
              <InputText
                v-else
                v-model="emailRegisterForm.email"
                autocomplete="email"
                placeholder="email@example.com"
                required
                type="email"
              />
            </label>
            <label class="field">
              <span>Mot de passe</span>
              <Password
                v-if="mode === 'login'"
                v-model="loginForm.password"
                :feedback="false"
                autocomplete="current-password"
                required
                toggle-mask
              />
              <Password
                v-else
                v-model="emailRegisterForm.password"
                autocomplete="new-password"
                required
                toggle-mask
              />
            </label>
            <RouterLink v-if="mode === 'login'" class="auth-forgot-link" to="/reset-password">
              Mot de passe oublié
            </RouterLink>
            <Button
              class="full-control"
              :label="mode === 'login' ? 'Se connecter' : 'Créer mon compte'"
              :loading="auth.loading"
              type="submit"
            />
          </form>
        </template>

        <form v-else class="auth-dialog-form" @submit.prevent="verifyPhoneOtp">
          <label class="field otp-field">
            <span>Code de vérification</span>
            <InputText
              v-model="otp"
              autocomplete="one-time-code"
              inputmode="numeric"
              maxlength="8"
              placeholder="000000"
              required
            />
          </label>
          <Button class="full-control" :loading="auth.loading" type="submit">
            <span class="auth-submit-content">
              <Check :size="19" aria-hidden="true" />
              Vérifier et continuer
            </span>
          </Button>
          <div class="otp-actions">
            <button type="button" @click="goBack">Modifier le numéro</button>
            <button
              type="button"
              :disabled="resendSeconds > 0 || auth.loading"
              @click="requestPhoneOtp"
            >
              {{ resendSeconds > 0 ? `Renvoyer dans ${resendSeconds}s` : 'Renvoyer le code' }}
            </button>
          </div>
        </form>
      </div>

      <div class="auth-dialog-legal">
        En continuant, tu acceptes les <RouterLink to="/terms">CGU</RouterLink> et la
        <RouterLink to="/privacy">politique de confidentialité</RouterLink>.
      </div>

      <footer class="auth-dialog-footer">
        <template v-if="mode === 'login'">
          <span>Tu n'as pas de compte ?</span>
          <button type="button" @click="switchMode('register')">S'inscrire</button>
        </template>
        <template v-else>
          <span>Tu as déjà un compte ?</span>
          <button type="button" @click="switchMode('login')">Se connecter</button>
        </template>
      </footer>
    </section>
  </Dialog>
</template>
