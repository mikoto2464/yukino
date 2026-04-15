<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row justify="center">
      <v-col cols="12" lg="4" md="6" sm="8">
        <v-card class="pa-6" elevation="2" rounded="xl">
          <h1 class="text-h6 font-weight-bold mb-2 text-primary">登录模拟</h1>
          <p class="text-medium-emphasis mb-6">无后端阶段，使用 Mock 用户进行鉴权演示。</p>

          <div class="d-flex flex-column ga-3">
            <v-btn :loading="loadingUser" color="primary" @click="loginAsUser">登录为普通用户</v-btn>
            <v-btn :loading="loadingAdmin" color="secondary" @click="loginAsAdmin">登录为管理员</v-btn>
          </div>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import {ref} from 'vue'
import {useRoute, useRouter} from 'vue-router'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const feedbackStore = useFeedbackStore()

const loadingUser = ref(false)
const loadingAdmin = ref(false)

function doneRedirect() {
  const redirect = route.query.redirect
  if (typeof redirect === 'string' && redirect.length > 0) {
    router.push(redirect)
    return
  }

  router.push({name: 'dashboard'})
}

function mockLogin(role: 'user' | 'admin') {
  const token = role === 'admin' ? 'mock-admin-token' : 'mock-user-token'

  authStore.setAuth(token, {
    id: role === 'admin' ? '1' : '2',
    name: role === 'admin' ? 'Yukino Admin' : 'Yukino User',
    roles: role === 'admin' ? ['user', 'admin'] : ['user']
  })

  feedbackStore.open({
    type: 'success',
    message: `已登录为${role === 'admin' ? '管理员' : '普通用户'}`
  })

  doneRedirect()
}

function loginAsUser() {
  loadingUser.value = true
  setTimeout(() => {
    mockLogin('user')
    loadingUser.value = false
  }, 500)
}

function loginAsAdmin() {
  loadingAdmin.value = true
  setTimeout(() => {
    mockLogin('admin')
    loadingAdmin.value = false
  }, 500)
}
</script>
