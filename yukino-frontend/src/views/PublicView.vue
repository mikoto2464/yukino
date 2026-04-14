<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row>
      <v-col cols="12" md="8" lg="7">
        <h1 class="text-h4 text-sm-h3 font-weight-bold mb-2 text-primary">Yukino Public</h1>
        <p class="text-medium-emphasis mb-6">公共页面，任何用户均可访问。</p>

        <v-btn v-if="authStore.isAuthenticated" color="primary" size="large" :to="{ name: 'dashboard' }">
          前往控制台
        </v-btn>
        <v-btn v-else color="primary" size="large" :to="{ name: 'login' }">
          登录
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { useAuthStore } from '../stores/auth'

const authStore = useAuthStore()
</script>
