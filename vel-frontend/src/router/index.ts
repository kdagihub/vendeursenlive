import { createRouter, createWebHistory } from 'vue-router'

import AppHomeView from '@/views/AppHomeView.vue'
import AuthView from '@/views/AuthView.vue'
import HomeView from '@/views/HomeView.vue'
import LiveRoomView from '@/views/LiveRoomView.vue'
import PrivacyView from '@/views/PrivacyView.vue'
import ResetPasswordView from '@/views/ResetPasswordView.vue'
import TermsView from '@/views/TermsView.vue'
import VerifyEmailView from '@/views/VerifyEmailView.vue'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      meta: {
        marketplace: true,
        title: 'VendeursEnLive',
        description:
          'Découvre les vendeurs en direct et commande les produits présentés pendant leurs lives.',
      },
    },
    {
      path: '/live/:id',
      name: 'live-room',
      component: LiveRoomView,
      meta: {
        marketplace: true,
        title: 'Live en direct',
      },
    },
    {
      path: '/login',
      name: 'login',
      component: AuthView,
    },
    {
      path: '/reset-password',
      name: 'reset-password',
      component: ResetPasswordView,
    },
    {
      path: '/verify-email',
      name: 'verify-email',
      component: VerifyEmailView,
    },
    {
      path: '/app',
      name: 'app-home',
      component: AppHomeView,
      meta: {
        requiresAuth: true,
      },
    },
    {
      path: '/terms',
      name: 'terms',
      component: TermsView,
      meta: {
        marketplace: true,
        title: 'Conditions générales d’utilisation',
        description:
          'Conditions applicables aux clients et vendeurs utilisant la marketplace VendeursEnLive.',
      },
    },
    {
      path: '/privacy',
      name: 'privacy',
      component: PrivacyView,
      meta: {
        marketplace: true,
        title: 'Politique de confidentialité',
        description:
          'Données collectées par VendeursEnLive, connexion TikTok, cookies et droits des utilisateurs.',
      },
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],
})

router.afterEach((to) => {
  const title = typeof to.meta.title === 'string' ? to.meta.title : 'VendeursEnLive'
  document.title = title === 'VendeursEnLive' ? title : `${title} | VendeursEnLive`

  const description = document.querySelector<HTMLMetaElement>('meta[name="description"]')
  if (description && typeof to.meta.description === 'string') {
    description.content = to.meta.description
  }
})

router.beforeEach(async (to) => {
  if (!to.meta.requiresAuth) {
    return true
  }

  const auth = useAuthStore()

  if (auth.isAuthenticated) {
    return true
  }

  const hasSession = await auth.fetchMe()

  if (!hasSession) {
    return {
      name: 'login',
      query: {
        redirect: to.fullPath,
      },
    }
  }

  return true
})

export default router
