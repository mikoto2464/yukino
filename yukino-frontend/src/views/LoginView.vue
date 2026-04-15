<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row justify="center">
      <v-col cols="12" lg="4" md="6" sm="8">
        <v-card class="pa-6" elevation="2" rounded="xl">
          <h1 class="text-h6 font-weight-bold mb-2 text-primary">登录</h1>
          <div class="d-flex flex-column ga-3">
            <v-btn :loading="loadingUser" color="primary" @click="loginByTelegram">使用Telegram登录</v-btn>
            <v-btn :loading="loadingAdmin" color="secondary" @click="loginAsAdmin">登录为管理员</v-btn>
          </div>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useFeedbackStore } from '../stores/feedback'
import http from '../api/axios'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const feedbackStore = useFeedbackStore()

const loadingUser = ref(false)
const loadingAdmin = ref(false)

const botId = '8691993144'

interface TelegramAuthUser {
  id: number
  nickname: string
  avatar_url: string
  role: string
  auth_stamp: string
}

onMounted(() => {
  if (!document.getElementById('telegram-widget-script')) {
    const script = document.createElement('script')
    script.id = 'telegram-widget-script'
    script.src = 'https://telegram.org/js/telegram-widget.js?22'
    script.async = true
    document.head.appendChild(script)
  }
})

function doneRedirect() {
  const redirect = route.query.redirect
  if (typeof redirect === 'string' && redirect.length > 0) {
    router.push(redirect)
    return
  }

  router.push({ name: 'dashboard' })
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

function loginByTelegram() {
  // 检查 JS 库是否加载完成
  if (!(window as any).Telegram || !(window as any).Telegram.Login) {
    feedbackStore.open({
      type: 'info',
      message: 'Telegram 组件尚未加载完成，请稍后再试'
    })
    return
  }

  loadingUser.value = true

  // 手动唤起 Telegram 授权弹窗
  ;(window as any).Telegram.Login.auth(
      {
        bot_id: botId,
        request_access: 'write',
      },
      async (data: any) => {
        // 弹窗关闭，取消按钮 loading 状态
        loadingUser.value = false

        if (!data) {
          feedbackStore.open({ type: 'info', message: '用户取消了授权' })
          return
        }

        try {
          const user = await http.post('/auth/telegram/callback', data) as TelegramAuthUser
          const role = user.role.toLowerCase() === 'admin' ? 'admin' : 'user'

          authStore.setSessionUser({
            id: String(user.id),
            name: user.nickname,
            roles: role === 'admin' ? ['user', 'admin'] : ['user'],
            nickname: user.nickname,
            avatarUrl: user.avatar_url,
            authStamp: user.auth_stamp
          })
          feedbackStore.open({ type: 'success', message: '登录成功' })
          doneRedirect()
        } catch (error) {
          console.error('Telegram login callback failed', error)
        }
      }
  )
}

function loginAsAdmin() {
  loadingAdmin.value = true
  setTimeout(() => {
    mockLogin('admin')
    loadingAdmin.value = false
  }, 500)
}
</script>
