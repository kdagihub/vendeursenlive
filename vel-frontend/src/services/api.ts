import axios from 'axios'

export const API_BASE_URL =
  (import.meta.env.VITE_API_BASE_URL as string | undefined)?.replace(/\/$/, '') ||
  resolveDefaultApiBaseUrl()

export const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json',
  },
  withCredentials: true,
})

const csrfProtectedPaths = ['/auth/refresh', '/auth/logout', '/auth/change-password']
const unsafeMethods = new Set(['post', 'put', 'patch', 'delete'])

export function getCookie(name: string): string | null {
  const prefix = `${name}=`
  const cookie = document.cookie.split('; ').find((entry) => entry.startsWith(prefix))

  if (!cookie) {
    return null
  }

  return decodeURIComponent(cookie.slice(prefix.length))
}

export async function ensureCsrfToken() {
  if (!getCookie('vel_csrf_token')) {
    await api.get('/auth/csrf')
  }

  if (!getCookie('vel_csrf_token')) {
    throw new Error(
      'Token CSRF inaccessible. Vérifie que le frontend et le backend utilisent le même host local.',
    )
  }
}

api.interceptors.request.use((config) => {
  const method = config.method?.toLowerCase()
  const url = config.url ?? ''

  if (method && unsafeMethods.has(method) && csrfProtectedPaths.some((path) => url.startsWith(path))) {
    const csrfToken = getCookie('vel_csrf_token')

    if (csrfToken) {
      config.headers.set('X-CSRF-Token', csrfToken)
    }
  }

  return config
})

function resolveDefaultApiBaseUrl(): string {
  if (typeof window === 'undefined') {
    return 'http://127.0.0.1:8080'
  }

  const { hostname, protocol } = window.location

  if (hostname === 'localhost' || hostname === '127.0.0.1') {
    return `${protocol}//${hostname}:8080`
  }

  if (hostname === 'vendeursenlive.shop') {
    return 'https://api.vendeursenlive.shop'
  }

  return 'http://127.0.0.1:8080'
}
