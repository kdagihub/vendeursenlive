import axios from 'axios'
import { ref } from 'vue'
import { defineStore } from 'pinia'

import { api, ensureCsrfToken } from '@/services/api'

export interface SellerLiveSession {
  id: string
  tiktok_live_url: string
  tiktok_username: string
  status: 'ongoing' | 'ended'
  created_at: string
  ended_at: string | null
}

export const useSellerLiveStore = defineStore('sellerLive', () => {
  const current = ref<SellerLiveSession | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  function clearError() {
    error.value = null
  }

  async function fetchCurrent() {
    loading.value = true
    error.value = null
    try {
      const response = await api.get<SellerLiveSession>('/seller/lives/current')
      current.value = response.status === 204 ? null : response.data
    } catch (requestError) {
      error.value = toPublicLiveError(requestError)
    } finally {
      loading.value = false
    }
  }

  async function start(tiktokLiveUrl: string) {
    loading.value = true
    error.value = null
    try {
      await ensureCsrfToken()
      const response = await api.post<SellerLiveSession>('/seller/lives', {
        tiktok_live_url: tiktokLiveUrl,
      })
      current.value = response.data
      return response.data
    } catch (requestError) {
      error.value = toPublicLiveError(requestError)
      throw requestError
    } finally {
      loading.value = false
    }
  }

  async function end() {
    if (!current.value) return
    loading.value = true
    error.value = null
    try {
      await ensureCsrfToken()
      await api.post(`/seller/lives/${current.value.id}/end`)
      current.value = null
    } catch (requestError) {
      error.value = toPublicLiveError(requestError)
      throw requestError
    } finally {
      loading.value = false
    }
  }

  return {
    clearError,
    current,
    end,
    error,
    fetchCurrent,
    loading,
    start,
  }
})

function toPublicLiveError(requestError: unknown): string {
  if (axios.isAxiosError(requestError)) {
    const status = requestError.response?.status
    const message = extractMessage(requestError.response?.data)

    if (status === 400 && message?.includes('shop name')) {
      return 'Renseigne d’abord le nom de ta boutique dans ton profil.'
    }
    if (status === 400) return 'Ce lien ne correspond pas à un LIVE TikTok valide.'
    if (status === 403) return 'Un compte vendeur vérifié est requis pour démarrer un LIVE.'
    if (status === 409) return 'Un LIVE est déjà actif pour cette boutique.'
    if (status === 503) return 'TikTok ne permet pas de vérifier ce lien pour le moment.'
  }

  return 'Impossible de mettre à jour le LIVE. Réessaie dans quelques instants.'
}

function extractMessage(data: unknown): string | null {
  if (!data || typeof data !== 'object') return null
  const message = (data as Record<string, unknown>).error
  return typeof message === 'string' ? message : null
}
