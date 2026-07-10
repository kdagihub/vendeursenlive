<script setup lang="ts">
import { reactive, ref } from 'vue'

import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()

const requestForm = reactive({
  identifier: '',
})

const confirmForm = reactive({
  resetToken: '',
  newPassword: '',
})

const requestSent = ref(false)
const passwordChanged = ref(false)

async function requestReset() {
  await auth.requestPasswordReset(requestForm.identifier.trim())
  requestSent.value = true
}

async function confirmReset() {
  await auth.confirmPasswordReset(confirmForm.resetToken.trim(), confirmForm.newPassword)
  confirmForm.resetToken = ''
  confirmForm.newPassword = ''
  passwordChanged.value = true
}
</script>

<template>
  <section class="narrow-view">
    <Card>
      <template #title>Réinitialiser le mot de passe</template>
      <template #content>
        <Message v-if="requestSent" severity="success">
          Si le compte existe, les instructions seront envoyées.
        </Message>
        <Message v-if="passwordChanged" severity="success">Mot de passe mis à jour.</Message>
        <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>

        <form class="auth-form" @submit.prevent="requestReset">
          <label class="field">
            <span>Email ou téléphone</span>
            <InputText
              v-model="requestForm.identifier"
              autocomplete="username"
              placeholder="awa@example.com ou +2250700000000"
              required
            />
          </label>

          <Button
            class="full-control"
            label="Recevoir les instructions"
            :loading="auth.loading"
            type="submit"
          />
        </form>

        <Divider />

        <form class="auth-form" @submit.prevent="confirmReset">
          <label class="field">
            <span>Token reçu</span>
            <InputText v-model="confirmForm.resetToken" required />
          </label>

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
            label="Confirmer"
            :loading="auth.loading"
            outlined
            type="submit"
          />
        </form>
      </template>
    </Card>
  </section>
</template>
