/* eslint-disable vue/multi-word-component-names */
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import Button from 'primevue/button'
import Card from 'primevue/card'
import Divider from 'primevue/divider'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Message from 'primevue/message'
import Password from 'primevue/password'
import SelectButton from 'primevue/selectbutton'

import App from './App.vue'
import router from './router'
import './assets/styles/main.css'

const app = createApp(App)
const prefersDark = window.matchMedia('(prefers-color-scheme: dark)')

function syncColorScheme(event: MediaQueryList | MediaQueryListEvent) {
  document.documentElement.classList.toggle('dark', event.matches)
}

syncColorScheme(prefersDark)
prefersDark.addEventListener('change', syncColorScheme)

app.use(createPinia())
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark',
    },
  },
})
app.use(router)

app.component('Button', Button)
app.component('Card', Card)
app.component('Divider', Divider)
app.component('Dialog', Dialog)
app.component('InputText', InputText)
app.component('Message', Message)
app.component('Password', Password)
app.component('SelectButton', SelectButton)

app.mount('#app')
