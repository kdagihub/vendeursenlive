<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const auth = useAuthStore()

const passwordForm = reactive({
  currentPassword: '',
  newPassword: '',
})

const successMessage = ref<string | null>(null)

async function refreshSession() {
  try {
    await auth.refresh()
    successMessage.value = 'Session rafraîchie.'
  } catch {
    successMessage.value = null
  }
}

async function logout() {
  await auth.logout()
  await router.push('/login')
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
</script>

<template>
  <section class="workspace-view">
    <div class="workspace-header">
      <div>
        <p class="eyebrow">Session active</p>
        <h1>Bienvenue dans ton espace {{ auth.accountLabel.toLowerCase() }}</h1>
      </div>

      <div class="action-row">
        <Button
          label="Rafraîchir"
          outlined
          :loading="auth.loading"
          type="button"
          @click="refreshSession"
        />
        <Button
          label="Déconnexion"
          severity="danger"
          :loading="auth.loading"
          type="button"
          @click="logout"
        />
      </div>
    </div>

    <Message v-if="successMessage" severity="success">{{ successMessage }}</Message>
    <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>

    <div class="workspace-grid">
      <Card>
        <template #title>Compte</template>
        <template #content>
          <dl class="session-list">
            <div>
              <dt>Identifiant</dt>
              <dd>{{ auth.user?.user_id }}</dd>
            </div>
            <div>
              <dt>Session</dt>
              <dd>{{ auth.user?.session_id ?? 'Non renseignée' }}</dd>
            </div>
            <div>
              <dt>Profil</dt>
              <dd>{{ auth.accountLabel }}</dd>
            </div>
          </dl>
        </template>
      </Card>

      <Card>
        <template #title>Changer le mot de passe</template>
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
              label="Mettre à jour"
              :loading="auth.loading"
              type="submit"
            />
          </form>
        </template>
      </Card>
    </div>
  </section>
</template>
