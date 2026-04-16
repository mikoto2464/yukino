<template>
  <v-layout class="min-h-screen">
    <v-app-bar color="primary" prominent>
      <RouterLink :to="{ name: 'home' }" class="brand-link ms-4">Yukino</RouterLink>

      <div class="d-flex align-center ga-1 ms-4 flex-wrap">
        <v-btn :to="{ name: 'admin' }" class="text-none" exact variant="text">卡密生成</v-btn>
        <v-btn :to="{ name: 'admin-projects' }" class="text-none" exact variant="text">项目管理</v-btn>
        <v-btn :to="{ name: 'admin-users' }" class="text-none" exact variant="text">用户管理</v-btn>
      </div>

      <template #append>
        <div class="d-flex align-center ga-2">
          <ThemeControls/>
          <v-btn :loading="logoutLoading" class="text-none" variant="text" @click="logout">退出登录</v-btn>
        </div>
      </template>
    </v-app-bar>

    <v-main class="yukino-main">
      <v-container class="pa-4 pa-md-6 yukino-content" fluid>
        <RouterView/>
      </v-container>
    </v-main>
  </v-layout>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {RouterLink, RouterView, useRouter} from 'vue-router'
import http from '../api/axios'
import ThemeControls from '../components/ThemeControls.vue'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

const router = useRouter()
const authStore = useAuthStore()
const feedbackStore = useFeedbackStore()
const logoutLoading = ref(false)

async function logout() {
  logoutLoading.value = true

  try {
    await http.delete('/user/logout')
    authStore.clearAuth()
    feedbackStore.open({type: 'success', message: '已退出登录'})
    await router.push({name: 'home'})
  } finally {
    logoutLoading.value = false
  }
}
</script>

<style scoped>
.brand-link {
  color: inherit;
  text-decoration: none;
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: 0.01em;
}

.brand-link:visited,
.brand-link:active,
.brand-link:hover,
.brand-link:focus {
  color: inherit;
  text-decoration: none;
}
</style>
