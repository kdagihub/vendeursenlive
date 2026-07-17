<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { RouterLink, useRoute } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const auth = useAuthStore()

const requestForm = reactive({
  email: '',
})

const confirmForm = reactive({
  newPassword: '',
})

const requestSent = ref(false)
const passwordChanged = ref(false)

const resetToken = computed(() => {
  const token = route.query.token

  return typeof token === 'string' ? token.trim() : ''
})

const hasResetToken = computed(() => resetToken.value.length > 0)

async function requestReset() {
  await auth.requestPasswordReset(requestForm.email.trim())
  requestSent.value = true
}

async function confirmReset() {
  await auth.confirmPasswordReset(resetToken.value, confirmForm.newPassword)
  confirmForm.newPassword = ''
  passwordChanged.value = true
}
</script>

<template>
  <section class="narrow-view">
    <Card>
      <template #title>
        {{ hasResetToken ? 'Nouveau mot de passe' : 'Réinitialiser le mot de passe' }}
      </template>
      <template #content>
        <Message v-if="requestSent" severity="success">
          Si le compte existe, un lien de réinitialisation vient d'être envoyé par email.
        </Message>
        <Message v-if="passwordChanged" severity="success">
          Mot de passe mis à jour. Tu peux maintenant te connecter.
        </Message>
        <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>

        <form v-if="!hasResetToken" class="auth-form" @submit.prevent="requestReset">
          <label class="field">
            <span>Email</span>
            <InputText
              v-model="requestForm.email"
              autocomplete="email"
              placeholder="email@example.com"
              required
              type="email"
            />
          </label>

          <Button
            class="full-control"
            label="Recevoir le lien"
            :loading="auth.loading"
            type="submit"
          />
        </form>

        <form v-else-if="!passwordChanged" class="auth-form" @submit.prevent="confirmReset">
          <label class="field">
            <span>Nouveau mot de passe</span>
            <Password
              v-model="confirmForm.newPassword"
              autocomplete="new-password"
              input-id="reset-password"
              required
              toggle-mask
            />
          </label>

          <Button
            class="full-control"
            label="Mettre à jour"
            :loading="auth.loading"
            type="submit"
          />
        </form>

        <p class="legal-note">
          <RouterLink to="/login">Retour à la connexion</RouterLink>
        </p>
      </template>
    </Card>
  </section>
</template>
