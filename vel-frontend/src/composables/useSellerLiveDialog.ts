import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'

export function useSellerLiveDialog() {
  const route = useRoute()
  const router = useRouter()

  const isLiveDialogOpen = computed(() => route.query.live === 'connect')

  function openLiveDialog() {
    return router.push({
      path: route.path,
      query: { ...route.query, live: 'connect' },
    })
  }

  function closeLiveDialog() {
    const query = { ...route.query }
    delete query.live
    return router.replace({ path: route.path, query })
  }

  return { closeLiveDialog, isLiveDialogOpen, openLiveDialog }
}
