<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

import { type AccountType, useAuthStore } from '@/stores/auth'

const router = useRouter()
const auth = useAuthStore()

const mode = ref<'login' | 'register'>('login')
const identifierMode = ref<'email' | 'phone'>('email')
const accountType = ref<AccountType>('customer')

const modeOptions = [
  { label: 'Connexion', value: 'login' },
  { label: 'Inscription', value: 'register' },
]

const identifierOptions = [
  { label: 'Email', value: 'email' },
  { label: 'Téléphone', value: 'phone' },
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
  defaultLocation: '',
  shopName: '',
  paymentLink: '',
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

    await router.push('/app')
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
      default_location:
        accountType.value === 'customer' ? registerForm.defaultLocation.trim() : undefined,
      shop_name: accountType.value === 'seller' ? registerForm.shopName.trim() : undefined,
      payment_link: accountType.value === 'seller' ? registerForm.paymentLink.trim() : undefined,
    })

    await router.push('/app')
  } catch {
    // The store exposes a user-facing error message.
  }
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
        <SelectButton
          v-model="mode"
          :allow-empty="false"
          :options="modeOptions"
          class="auth-mode-toggle"
          option-label="label"
          option-value="value"
        />

        <Message v-if="auth.error" severity="error" class="form-message">
          {{ auth.error }}
        </Message>

        <form v-if="mode === 'login'" class="auth-form" @submit.prevent="submitLogin">
          <label class="field">
            <span>Email ou téléphone</span>
            <InputText
              v-model="loginForm.identifier"
              autocomplete="username"
              placeholder="awa@example.com ou +2250700000000"
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

          <Button
            class="full-control"
            label="Se connecter"
            :loading="auth.loading"
            type="submit"
          />
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

          <label v-if="accountType === 'customer'" class="field">
            <span>Localisation par défaut</span>
            <InputText v-model="registerForm.defaultLocation" placeholder="Cocody" />
          </label>

          <template v-else>
            <label class="field">
              <span>Nom de la boutique</span>
              <InputText v-model="registerForm.shopName" placeholder="Boutique Awa" required />
            </label>

            <label class="field">
              <span>Lien de paiement</span>
              <InputText
                v-model="registerForm.paymentLink"
                placeholder="https://pay.wave.com/..."
                type="url"
              />
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
          label="Continuer avec TikTok"
          outlined
          severity="contrast"
          type="button"
          @click="auth.startTikTokLogin"
        />

        <p class="legal-note">
          En continuant, tu acceptes les <RouterLink to="/terms">CGU</RouterLink> et la
          <RouterLink to="/privacy">politique de confidentialité</RouterLink>.
        </p>
      </template>
    </Card>
  </section>
</template>
