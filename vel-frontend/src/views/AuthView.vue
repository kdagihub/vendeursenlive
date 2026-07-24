<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import tiktokLogo from '@/assets/img/tiktok-icon-official.png'
import { type AccountType, useAuthStore } from '@/stores/auth'

const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

const mode = ref<'login' | 'register'>(route.query.mode === 'register' ? 'register' : 'login')
const identifierMode = ref<'email' | 'phone'>('phone')
const accountType = ref<AccountType>('customer')

const identifierOptions = [
  { label: 'Téléphone', value: 'phone' },
  { label: 'Email', value: 'email' },
]

const accountOptions = [
  { label: 'Client', value: 'customer' },
  { label: 'Vendeur', value: 'seller' },
]

const loginForm = reactive({
  identifier: '',
  password: '',
})

const registerForm = reactive({
  fullName: '',
  email: '',
  phoneNumber: '',
  password: '',
  shopName: '',
})

const title = computed(() =>
  mode.value === 'login' ? 'Connecte-toi à VendeursEnLive' : 'Créer un compte',
)

const subtitle = computed(() =>
  mode.value === 'login'
    ? 'Gère tes achats, tes ventes et tes lives depuis un seul espace.'
    : 'Rejoins VendeursEnLive comme client ou vendeur.',
)

async function submitLogin() {
  try {
    await auth.login({
      identifier: loginForm.identifier.trim(),
      password: loginForm.password,
    })

    await router.push(authDestination())
  } catch {
    // The store exposes a user-facing error message.
  }
}

async function submitRegister() {
  try {
    await auth.register({
      full_name: registerForm.fullName.trim(),
      email: identifierMode.value === 'email' ? registerForm.email.trim() : undefined,
      phone_number: identifierMode.value === 'phone' ? registerForm.phoneNumber.trim() : undefined,
      password: registerForm.password,
      account_type: accountType.value,
      shop_name: accountType.value === 'seller' ? registerForm.shopName.trim() : undefined,
    })

    await router.push(authDestination())
  } catch {
    // The store exposes a user-facing error message.
  }
}

function authDestination() {
  const redirect = typeof route.query.redirect === 'string' ? route.query.redirect : '/app'
  return redirect.startsWith('/') && !redirect.startsWith('//') ? redirect : '/app'
}

function switchMode(nextMode: 'login' | 'register') {
  mode.value = nextMode
  auth.error = null
}
</script>

<template>
  <section class="auth-view">
    <Card class="auth-card">
      <template #title>
        <div class="auth-title">
          <h1>{{ title }}</h1>
          <p>{{ subtitle }}</p>
        </div>
      </template>

      <template #content>
        <Message v-if="auth.error" severity="error" class="form-message">
          {{ auth.error }}
        </Message>

        <form v-if="mode === 'login'" class="auth-form" @submit.prevent="submitLogin">
          <label class="field">
            <span>Téléphone ou email</span>
            <InputText
              v-model="loginForm.identifier"
              autocomplete="username"
              placeholder="+2250700000000 ou email@example.com"
              required
            />
          </label>

          <label class="field">
            <span>Mot de passe</span>
            <Password
              v-model="loginForm.password"
              :feedback="false"
              autocomplete="current-password"
              input-id="login-password"
              required
              toggle-mask
            />
          </label>

          <div class="form-row">
            <RouterLink to="/reset-password">Mot de passe oublié</RouterLink>
          </div>

          <Button class="full-control" label="Se connecter" :loading="auth.loading" type="submit" />
        </form>

        <form v-else class="auth-form" @submit.prevent="submitRegister">
          <SelectButton
            v-model="identifierMode"
            :allow-empty="false"
            :options="identifierOptions"
            class="form-toggle"
            option-label="label"
            option-value="value"
          />

          <label class="field">
            <span>Nom complet</span>
            <InputText
              v-model="registerForm.fullName"
              autocomplete="name"
              placeholder="Awa Kouamé"
              required
            />
          </label>

          <label v-if="identifierMode === 'email'" class="field">
            <span>Email</span>
            <InputText
              v-model="registerForm.email"
              autocomplete="email"
              placeholder="awa@example.com"
              required
              type="email"
            />
          </label>

          <label v-else class="field">
            <span>Téléphone</span>
            <InputText
              v-model="registerForm.phoneNumber"
              autocomplete="tel"
              placeholder="+2250700000000"
              required
            />
          </label>

          <label class="field">
            <span>Mot de passe</span>
            <Password
              v-model="registerForm.password"
              autocomplete="new-password"
              input-id="register-password"
              required
              toggle-mask
            />
          </label>

          <SelectButton
            v-model="accountType"
            :allow-empty="false"
            :options="accountOptions"
            class="form-toggle"
            option-label="label"
            option-value="value"
          />

          <template v-if="accountType === 'seller'">
            <label class="field">
              <span>Nom de la boutique</span>
              <InputText v-model="registerForm.shopName" placeholder="Boutique Awa" required />
            </label>
          </template>

          <Button
            class="full-control"
            label="Créer mon compte"
            :loading="auth.loading"
            type="submit"
          />
        </form>

        <Divider align="center">ou</Divider>

        <Button
          class="full-control tiktok-button"
          aria-label="Continuer avec TikTok"
          type="button"
          @click="auth.startTikTokLogin"
        >
          <span class="tiktok-button-content">
            <img class="tiktok-mark" :src="tiktokLogo" alt="" aria-hidden="true" />
            <span>Continuer avec TikTok</span>
          </span>
        </Button>

        <p class="legal-note">
          En continuant, tu acceptes les <RouterLink to="/terms">CGU</RouterLink> et la
          <RouterLink to="/privacy">politique de confidentialité</RouterLink>.
        </p>

        <div class="auth-switch">
          <template v-if="mode === 'login'">
            <span>Tu n'as pas de compte ?</span>
            <button type="button" @click="switchMode('register')">S'inscrire</button>
          </template>
          <template v-else>
            <span>Tu as déjà un compte ?</span>
            <button type="button" @click="switchMode('login')">Se connecter</button>
          </template>
        </div>
      </template>
    </Card>
  </section>
</template>
