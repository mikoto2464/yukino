<template>
  <v-layout class="min-h-screen">
    <v-app-bar variant="flat" border="b">
      <RouterLink :to="{ name: 'home' }" class="brand-link ms-3">Yukino</RouterLink>

      <div class="d-flex align-center ga-1 ms-4 flex-wrap">
        <v-btn :to="{ name: 'admin' }" variant="text" class="text-none">卡密生成</v-btn>
        <v-btn :to="{ name: 'admin-projects' }" variant="text" class="text-none">项目管理</v-btn>
        <v-btn :to="{ name: 'admin-users' }" variant="text" class="text-none">用户管理</v-btn>
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
</script>

<style scoped>
.brand-link {
  color: rgb(var(--v-theme-primary));
  text-decoration: none;
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: 0.01em;
}

.brand-link:hover,
.brand-link:focus,
.brand-link:visited,
.brand-link:active {
  color: rgb(var(--v-theme-primary));
  text-decoration: none;
}
</style>
