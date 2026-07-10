import { createRouter, createWebHistory } from 'vue-router'

import AppHomeView from '@/views/AppHomeView.vue'
import AuthView from '@/views/AuthView.vue'
import PrivacyView from '@/views/PrivacyView.vue'
import ResetPasswordView from '@/views/ResetPasswordView.vue'
import TermsView from '@/views/TermsView.vue'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/login',
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
    },
    {
      path: '/privacy',
      name: 'privacy',
      component: PrivacyView,
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/login',
    },
  ],
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
