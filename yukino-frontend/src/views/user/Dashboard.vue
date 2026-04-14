<template>
  <v-row class="ga-0">
    <v-col cols="12" md="6" lg="4" class="mb-4">
      <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5 fill-height">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">用户信息</h2>

        <div class="d-flex align-center mb-4">
          <v-avatar size="80" color="primary" variant="tonal" class="me-4">
            <span class="text-h6 font-weight-bold">{{ user.initials }}</span>
          </v-avatar>

          <div>
            <div class="text-subtitle-1 font-weight-bold">{{ user.name }}</div>
            <div class="text-body-2 text-medium-emphasis">{{ user.email }}</div>
            <div class="text-body-2 text-medium-emphasis">绑定 Telegram: {{ user.telegram }}</div>
          </div>
        </div>

        <div class="d-flex ga-2 flex-wrap">
          <v-btn color="error" variant="tonal" :loading="unbindLoading" @click="handleUnbind">解绑</v-btn>
          <v-btn color="primary" variant="tonal" :loading="rebindLoading" @click="handleRebind">换绑</v-btn>
        </div>
      </v-card>
    </v-col>

    <v-col cols="12" md="6" lg="4" class="mb-4">
      <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5 fill-height">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">在线设备</h2>

        <v-list lines="two" class="pa-0 mb-2 bg-transparent">
          <v-list-item
            v-for="device in devices"
            :key="device.id"
            :title="device.name"
            :subtitle="`最后心跳: ${device.lastSeen}`"
            class="px-0"
          >
            <template #prepend>
              <v-avatar color="secondary" variant="tonal" size="36">
                <v-icon :icon="device.icon" />
              </v-avatar>
            </template>
            <template #append>
              <v-btn
                size="small"
                color="error"
                variant="text"
                :loading="kickLoadingId === device.id"
                @click="kickDevice(device.id)"
              >
                下线
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
      </v-card>
    </v-col>

    <v-col cols="12" md="6" lg="4" class="mb-4">
      <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5 fill-height">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">卡密激活</h2>

        <v-form @submit.prevent="submitActivation">
          <v-text-field
            v-model="activationCode"
            label="请输入卡密"
            variant="outlined"
            rounded="lg"
            color="primary"
            class="mb-3"
            :disabled="activationLoading"
          />

          <v-btn
            block
            size="large"
            color="primary"
            rounded="lg"
            elevation="1"
            type="submit"
            :loading="activationLoading"
          >
            激活
          </v-btn>
        </v-form>
      </v-card>
    </v-col>

    <v-col cols="12">
      <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">可用项目</h2>

        <v-row>
          <v-col cols="12" sm="6" md="4" v-for="project in projects" :key="project.id" class="mb-2">
            <v-card rounded="xl" elevation="2" variant="flat" border class="pa-4">
              <div class="text-subtitle-1 font-weight-bold mb-1">{{ project.name }}</div>
              <div class="text-body-2 text-medium-emphasis mb-3">{{ project.description }}</div>
              <v-chip color="primary" size="small" variant="tonal">{{ project.status }}</v-chip>
            </v-card>
          </v-col>
        </v-row>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useFeedbackStore } from '../../stores/feedback'

interface DeviceItem {
  id: string
  name: string
  lastSeen: string
  icon: string
}

interface ProjectItem {
  id: string
  name: string
  description: string
  status: string
}

const feedback = useFeedbackStore()

const user = {
  initials: 'YU',
  name: 'Yukino User',
  email: 'user@yukino.app',
  telegram: '@yukino_user'
}

const devices = ref<DeviceItem[]>([
  { id: 'dev-1', name: 'Windows Desktop', lastSeen: '刚刚', icon: 'mdi-monitor' },
  { id: 'dev-2', name: 'MacBook Pro', lastSeen: '2 分钟前', icon: 'mdi-laptop' },
  { id: 'dev-3', name: 'Android Phone', lastSeen: '5 分钟前', icon: 'mdi-cellphone' }
])

const projects = ref<ProjectItem[]>([
  { id: 'proj-1', name: 'Yukino Agent', description: '设备侧守护程序与自动更新组件。', status: '可激活' },
  { id: 'proj-2', name: 'Yukino Launcher', description: '桌面分发启动器，支持多版本并行。', status: '可下载' },
  { id: 'proj-3', name: 'Yukino Monitor', description: '运行态监控与远程运维工具。', status: '内测开放' }
])

const unbindLoading = ref(false)
const rebindLoading = ref(false)
const kickLoadingId = ref('')
const activationCode = ref('')
const activationLoading = ref(false)

function handleUnbind() {
  unbindLoading.value = true
  setTimeout(() => {
    unbindLoading.value = false
    feedback.open({ type: 'info', message: '解绑请求已提交' })
  }, 600)
}

function handleRebind() {
  rebindLoading.value = true
  setTimeout(() => {
    rebindLoading.value = false
    feedback.open({ type: 'success', message: '换绑验证流程已启动' })
  }, 600)
}

function kickDevice(deviceId: string) {
  kickLoadingId.value = deviceId
  setTimeout(() => {
    devices.value = devices.value.filter((item) => item.id !== deviceId)
    kickLoadingId.value = ''
    feedback.open({ type: 'success', message: '设备已下线' })
  }, 500)
}

function submitActivation() {
  if (!activationCode.value.trim()) {
    feedback.open({ type: 'error', message: '请输入有效卡密' })
    return
  }

  activationLoading.value = true
  setTimeout(() => {
    activationLoading.value = false
    feedback.open({ type: 'success', message: '卡密激活成功（Mock）' })
    activationCode.value = ''
  }, 900)
}
</script>
