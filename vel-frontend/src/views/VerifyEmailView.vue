<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { RouterLink, useRoute } from 'vue-router'

import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const auth = useAuthStore()
const verified = ref(false)
const invalidLink = ref(false)

const verificationToken = computed(() => {
  const token = route.query.token
  return typeof token === 'string' ? token.trim() : ''
})

onMounted(async () => {
  if (!verificationToken.value) {
    invalidLink.value = true
    return
  }

  try {
    await auth.confirmEmailVerification(verificationToken.value)
    verified.value = true
  } catch {
    invalidLink.value = true
  }
})
</script>

<template>
  <section class="narrow-view">
    <Card>
      <template #title>Vérification de l'email</template>
      <template #content>
        <Message v-if="auth.loading" severity="info">Vérification en cours…</Message>
        <Message v-else-if="verified" severity="success">
          Ton adresse email est vérifiée. Toutes les fonctionnalités sont maintenant accessibles.
        </Message>
        <Message v-else-if="invalidLink" severity="error">
          Ce lien est invalide ou expiré. Demande un nouveau lien depuis ton espace.
        </Message>

        <p class="legal-note">
          <RouterLink :to="auth.isAuthenticated ? '/profile' : '/login'">
            {{ auth.isAuthenticated ? 'Retour à mon profil' : 'Retour à la connexion' }}
          </RouterLink>
        </p>
      </template>
    </Card>
  </section>
</template>
