<template>
  <v-container class="pa-4 pa-sm-6" fluid>
    <v-row justify="center">
      <v-col cols="12" lg="4" md="6" sm="8">
        <v-card class="pa-6" elevation="2" rounded="xl">
          <h1 class="text-h6 font-weight-bold mb-2 text-primary">登录</h1>
          <div class="d-flex flex-column ga-3">
            <v-btn :loading="loadingTelegram" color="primary" @click="loginByTelegram">使用Telegram登录</v-btn>
            <v-btn :loading="loadingQQ" color="primary" @click="loginByQQ">使用QQ登录</v-btn>
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

interface TelegramAuth {
  id: number
  nickname: string
  avatar_url: string
  role: string
  auth_stamp: string
}

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const feedbackStore = useFeedbackStore()

const loadingTelegram = ref(false)
const loadingQQ = ref(false)

const botId = '8691993144'

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
  if (!(window as any).Telegram || !(window as any).Telegram.Login) {
    feedbackStore.open({
      type: 'info',
      message: 'Telegram 组件尚未加载完成，请稍后再试'
    })
    return
  }

  loadingTelegram.value = true

  ;(window as any).Telegram.Login.auth(
      {
        bot_id: botId,
        request_access: 'write',
      },
      async (data: any) => {
        loadingTelegram.value = false

        if (!data) {
          feedbackStore.open({ type: 'info', message: '用户取消了授权' })
          return
        }

        try {
          const user = await http.post('/auth/telegram/callback', data) as TelegramAuth
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

function loginByQQ() {
  loadingQQ.value = true
  setTimeout(() => {
    mockLogin('user')
    loadingQQ.value = false
  }, 500)
}
</script>
