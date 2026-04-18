<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row>
      <v-col cols="12" md="6" sm="8">
        <v-card class="pa-6 mb-4" elevation="2" rounded="xl">
          <h1 class="text-h5 font-weight-bold mb-2 text-primary">用户控制台</h1>
          <p class="text-medium-emphasis mb-6">当前页面需要登录访问。</p>

          <v-chip class="me-2" color="primary" variant="tonal">{{ authStore.user?.name ?? '未登录' }}</v-chip>
          <v-chip color="secondary" variant="tonal">角色: {{ authStore.roles.join(', ') || '无' }}</v-chip>

          <div class="d-flex ga-3 mt-6 flex-wrap">
            <v-btn :to="{ name: 'admin' }" color="primary">前往管理员页</v-btn>
            <v-btn :loading="logoutLoading" color="error" variant="tonal" @click="logout">退出登录</v-btn>
          </div>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {useRouter} from 'vue-router'
import http from '../api/axios'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

const router = useRouter()
const authStore = useAuthStore()
const feedbackStore = useFeedbackStore()
const logoutLoading = ref(false)

async function logout() {
  logoutLoading.value = true
  try {
    await http.delete('/user/session')
    authStore.clearAuth()
    feedbackStore.open({
      type: 'success',
      message: '已安全退出'
    })
    await router.push({name: 'home'})
  } finally {
    logoutLoading.value = false
  }
}
</script>
