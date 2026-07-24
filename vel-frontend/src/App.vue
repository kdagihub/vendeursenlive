<script setup lang="ts">
import { computed, ref } from 'vue'
import { RouterLink, RouterView, useRoute } from 'vue-router'

import logo from '@/assets/img/logo_vel_soft.png'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
const route = useRoute()
const usesMarketplaceLayout = computed(() => route.meta.marketplace === true)
const verificationSent = ref(false)
const verificationDialogVisible = ref(false)

async function resendVerificationEmail() {
  try {
    await auth.requestEmailVerification()
    verificationSent.value = true
  } catch {
    verificationSent.value = false
  }
}
</script>

<template>
  <RouterView v-if="usesMarketplaceLayout" />

  <div v-else class="app-shell">
    <header class="topbar">
      <RouterLink class="brand-link" to="/" aria-label="VendeursEnLive">
        <img class="brand-logo" :src="logo" alt="" />
        <span class="brand-name">
          <span>VendeursEn</span><span class="brand-name-accent">Live</span>
        </span>
      </RouterLink>

      <nav class="topnav" aria-label="Navigation principale">
        <RouterLink to="/terms">CGU</RouterLink>
        <RouterLink to="/privacy">Confidentialité</RouterLink>
        <RouterLink v-if="auth.isAuthenticated" to="/app">Mon espace</RouterLink>
      </nav>
    </header>

    <aside v-if="auth.requiresEmailVerification" class="verification-banner" role="status">
      <div>
        <strong>Compte email non vérifié</strong>
        <span>
          Vérifie ton email pour commander pendant les lives et utiliser les actions vendeur.
        </span>
      </div>
      <Button
        label="Vérifier mon email"
        outlined
        size="small"
        type="button"
        @click="verificationDialogVisible = true"
      />
    </aside>

    <Dialog
      v-model:visible="verificationDialogVisible"
      :draggable="false"
      header="Vérifie ton adresse email"
      modal
      class="verification-dialog"
    >
      <p>Le lien reçu par email déverrouille les commandes client et les actions vendeur.</p>
      <Message v-if="verificationSent" severity="success">
        Un nouveau lien vient d'être envoyé.
      </Message>
      <Message v-if="auth.error" severity="error">{{ auth.error }}</Message>

      <template #footer>
        <Button
          label="Plus tard"
          severity="secondary"
          text
          type="button"
          @click="verificationDialogVisible = false"
        />
        <Button
          :label="verificationSent ? 'Email envoyé' : 'Renvoyer le lien'"
          :disabled="verificationSent"
          :loading="auth.loading"
          type="button"
          @click="resendVerificationEmail"
        />
      </template>
    </Dialog>

    <main class="main-surface">
      <RouterView />
    </main>

    <footer class="footer">
      <span>© 2026 VendeursEnLive</span>
      <span>Live commerce pour vendeurs et clients.</span>
    </footer>
  </div>
</template>
