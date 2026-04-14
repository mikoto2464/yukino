<template>
  <v-layout class="min-h-screen">
    <v-app-bar color="primary" prominent>
      <RouterLink :to="{ name: 'home' }" class="brand-link ms-3">Yukino</RouterLink>

      <div class="d-flex align-center ga-2 ms-4" v-if="authStore.isAdmin">
        <v-btn :to="{ name: 'admin' }" variant="text" class="text-none">前往管理员页面</v-btn>
      </div>

      <template #append>
        <ThemeControls />
      </template>
    </v-app-bar>

    <v-main class="yukino-main">
      <v-container fluid class="pa-4 pa-md-6 bg-background fill-height yukino-content">
        <RouterView />
      </v-container>
    </v-main>
  </v-layout>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import ThemeControls from '../components/ThemeControls.vue'
import { useAuthStore } from '../stores/auth'

const authStore = useAuthStore()
</script>

<style scoped>
.brand-link {
  color: inherit;
  text-decoration: none;
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: 0.01em;
}

.brand-link:hover,
.brand-link:focus,
.brand-link:visited,
.brand-link:active {
  color: inherit;
  text-decoration: none;
}
</style>
