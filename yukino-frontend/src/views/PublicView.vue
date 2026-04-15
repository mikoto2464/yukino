<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row>
      <v-col cols="12" lg="7" md="8">
        <h1 class="text-h4 text-sm-h3 font-weight-bold mb-2 text-primary">Yukino Public</h1>
        <p class="text-medium-emphasis mb-6">公共页面，任何用户均可访问。</p>

        <v-btn v-if="authStore.isAuthenticated" :to="{ name: 'dashboard' }" color="primary" size="large">
          前往控制台
        </v-btn>
        <v-btn v-else :to="{ name: 'login' }" color="primary" size="large">
          登录
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import {useAuthStore} from '../stores/auth'

const authStore = useAuthStore()
</script>
