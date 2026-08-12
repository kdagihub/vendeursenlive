import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'

export type AuthDialogMode = 'login' | 'register'

export function safeInternalRedirect(value: unknown): string | null {
  if (typeof value !== 'string') return null
  return value.startsWith('/') && !value.startsWith('//') ? value : null
}

export function resolveAuthDestination(value: unknown): string {
  const redirect = safeInternalRedirect(value)
  if (!redirect) return '/'

  const pathname = redirect.split(/[?#]/, 1)[0]
  if (pathname === '/app' || pathname === '/profile') return '/'

  return redirect
}

export function useAuthDialog() {
  const route = useRoute()
  const router = useRouter()

  const mode = computed<AuthDialogMode>(() =>
    route.query.auth === 'register' ? 'register' : 'login',
  )
  const isOpen = computed(() => route.query.auth === 'login' || route.query.auth === 'register')

  function openAuthDialog(nextMode: AuthDialogMode = 'login', redirect?: string) {
    return router.push({
      path: route.path,
      query: {
        ...route.query,
        auth: nextMode,
        ...(redirect ? { redirect } : {}),
      },
    })
  }

  function setAuthDialogMode(nextMode: AuthDialogMode) {
    return router.replace({
      path: route.path,
      query: {
        ...route.query,
        auth: nextMode,
      },
    })
  }

  function closeAuthDialog() {
    const query = { ...route.query }
    delete query.auth
    delete query.account
    delete query.redirect
    return router.replace({ path: route.path, query })
  }

  return {
    closeAuthDialog,
    isOpen,
    mode,
    openAuthDialog,
    setAuthDialogMode,
  }
}
