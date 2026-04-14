import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'
import { initThemeEngine } from './theme/themeEngine'

import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import 'nprogress/nprogress.css'
import './styles/main.css'

await initThemeEngine(vuetify.theme)

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(vuetify)
app.use(router)

app.mount('#app')

const boot = document.getElementById('boot-loading')
if (boot) {
  boot.classList.add('hidden')
  window.setTimeout(() => boot.remove(), 260)
}
