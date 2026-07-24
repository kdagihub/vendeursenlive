<script setup lang="ts">
import BrandMark from '@/components/BrandMark.vue'

withDefaults(
  defineProps<{
    mobileNavOffset?: boolean
  }>(),
  {
    mobileNavOffset: false,
  },
)
</script>

<template>
  <footer
    class="mt-14 bg-[#17121f] px-4 pt-9 text-white sm:px-6 lg:mt-18 lg:px-8"
    :class="mobileNavOffset ? 'pb-24 lg:pb-9' : 'pb-9'"
  >
    <div class="mx-auto grid w-full max-w-[1500px] gap-8 md:grid-cols-[1.3fr_1fr_1fr]">
      <div class="max-w-md">
        <div class="inline-flex rounded-md bg-white px-2 py-1">
          <BrandMark />
        </div>
        <p class="mt-4 text-sm leading-6 text-slate-300">
          La marketplace ivoirienne qui relie les directs des vendeurs à un parcours de commande
          simple et transparent.
        </p>
      </div>

      <nav aria-label="Informations légales">
        <p class="text-xs font-black text-white uppercase">Informations légales</p>
        <div class="mt-3 grid gap-2.5 text-sm">
          <RouterLink class="w-fit text-slate-300 hover:text-white" to="/terms">
            Conditions générales d’utilisation
          </RouterLink>
          <RouterLink class="w-fit text-slate-300 hover:text-white" to="/privacy">
            Politique de confidentialité
          </RouterLink>
        </div>
      </nav>

      <div>
        <p class="text-xs font-black text-white uppercase">Nous contacter</p>
        <div class="mt-3 flex flex-wrap gap-x-4 gap-y-2 text-sm">
          <a class="text-slate-300 hover:text-white" href="mailto:contact@vendeursenlive.shop">
            contact@vendeursenlive.shop
          </a>
          <a class="text-slate-300 hover:text-white" href="tel:+2250797969394">
            +225 07 97 96 93 94
          </a>
        </div>
        <p class="mt-2 text-sm text-slate-400">Côte d’Ivoire</p>
      </div>
    </div>

    <div
      class="mx-auto mt-8 flex w-full max-w-[1500px] flex-wrap justify-between gap-2 border-t border-white/12 pt-5 text-xs text-slate-400"
    >
      <span>© 2026 VendeursEnLive</span>
      <span>Un service exploité par CIACEMS.</span>
    </div>
  </footer>
</template>
