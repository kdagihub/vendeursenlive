<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'

import logo from '@/assets/img/logo_v1.png'
import { useAuthStore } from '@/stores/auth'

const auth = useAuthStore()
</script>

<template>
  <div class="app-shell">
    <header class="topbar">
      <RouterLink class="brand-link" to="/login" aria-label="VendeursEnLive">
        <img class="brand-logo" :src="logo" alt="" />
        <span class="brand-name">VendeursEnLive</span>
      </RouterLink>

      <nav class="topnav" aria-label="Navigation principale">
        <RouterLink to="/terms">CGU</RouterLink>
        <RouterLink to="/privacy">Confidentialité</RouterLink>
        <RouterLink v-if="auth.isAuthenticated" to="/app">Mon espace</RouterLink>
      </nav>
    </header>

    <main class="main-surface">
      <RouterView />
    </main>

    <footer class="footer">
      <span>© 2026 VendeursEnLive</span>
      <span>Live commerce pour vendeurs et clients.</span>
    </footer>
  </div>
</template>
