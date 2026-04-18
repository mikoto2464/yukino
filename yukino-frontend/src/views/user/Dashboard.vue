<template>
  <v-row class="ga-0">
    <v-col class="mb-4" cols="12" md="6">
      <section class="panel-surface h-100">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">用户信息</h2>

        <div class="d-flex align-center mb-4">
          <v-avatar class="me-4" color="primary" size="80" variant="tonal">
            <v-img v-if="user.avatarUrl" :src="user.avatarUrl" alt="avatar" cover/>
            <span v-else class="text-h6 font-weight-bold">{{ user.initials }}</span>
          </v-avatar>

          <div>
            <div class="text-subtitle-1 font-weight-bold">{{ user.name }}</div>
            <div class="text-body-2 text-medium-emphasis">用户编号: {{ user.userId }}</div>
            <div class="text-body-2 text-medium-emphasis">绑定 Telegram: {{ user.telegram }}</div>
          </div>
        </div>

        <div class="d-flex ga-2 flex-wrap">
          <v-btn :loading="unbindLoading" color="error" variant="tonal" @click="handleUnbind">解绑</v-btn>
          <v-btn :loading="rebindLoading" color="primary" variant="tonal" @click="handleRebind">换绑</v-btn>
        </div>
      </section>
    </v-col>

    <v-col class="mb-4" cols="12" md="6">
      <section class="panel-surface h-100">
        <h2 class="text-h6 font-weight-bold mb-4 text-primary">设备列表</h2>

        <v-list
            :class="{'device-list-scroll': devices.length > 3}"
            class="pa-0 mb-3 bg-transparent"
            lines="two"
        >
          <v-list-item
              v-for="device in devices"
              :key="device.id"
              :subtitle="`最后心跳: ${device.lastSeen}`"
              :title="device.name"
              class="px-0"
          >
            <template #prepend>
              <v-avatar color="secondary" size="36" variant="tonal">
                <v-icon :icon="device.icon"/>
              </v-avatar>
            </template>
            <template #append>
              <v-btn
                  :loading="kickLoadingId === device.id"
                  color="error"
                  size="small"
                  variant="text"
                  @click="deleteDevice(device.id)"
              >
                下线
              </v-btn>
            </template>
          </v-list-item>
        </v-list>

        <v-row class="align-end">
          <v-col cols="12" sm="8">
            <v-text-field
                v-model="bindCode"
                color="primary"
                density="comfortable"
                hide-details
                label="输入绑定代码"
                rounded="lg"
                variant="outlined"
            />
          </v-col>
          <v-col cols="12" sm="4">
            <v-btn :loading="bindLoading" block color="primary" rounded="lg" size="large" @click="createDevice">新增设备
            </v-btn>
          </v-col>
        </v-row>
      </section>
    </v-col>

    <v-col cols="12">
      <section class="panel-surface">
        <div class="d-flex flex-column flex-md-row align-md-center justify-space-between mb-4 ga-3">
          <h2 class="text-h6 font-weight-bold text-primary">可用项目</h2>

          <div class="d-flex flex-column flex-sm-row ga-2" style="min-width: min(100%, 680px)">
            <v-text-field
                v-model="activationCode"
                class="flex-grow-1"
                color="primary"
                density="comfortable"
                hide-details
                label="输入卡密"
                rounded="lg"
                variant="outlined"
            />
            <v-btn :loading="activationLoading" color="primary" rounded="lg" size="large" @click="submitActivation">
              激活卡密
            </v-btn>
          </div>
        </div>

        <div class="d-flex justify-end mb-3">
          <v-text-field
              v-model="projectSearch"
              class="search-field"
              clearable
              density="compact"
              hide-details
              placeholder="搜索项目 / 状态"
              prepend-inner-icon="mdi-magnify"
              style="max-width: 340px"
              variant="solo-filled"
          />
        </div>

        <v-data-table :headers="projectHeaders" :items="filteredProjects" :items-per-page="8" class="elevation-0">
          <template v-slot:[`item.status`]="{ item }">
            <v-chip color="primary" size="small" variant="tonal">{{ item.status }}</v-chip>
          </template>
        </v-data-table>
      </section>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {computed, ref} from 'vue'
import http from '../../api/axios'
import {useAuthStore} from '../../stores/auth'
import {useFeedbackStore} from '../../stores/feedback'

interface DeviceItem {
  id: string
  name: string
  lastSeen: string
  icon: string
}

interface ProjectRow {
  id: string
  name: string
  status: string
  version: string
  expiresAt: string
}

interface Device {
  hardware_id: string
  user_id: number
  name: string
  last_seen: number
}

const authStore = useAuthStore()
const feedback = useFeedbackStore()

const user = computed(() => {
  const current = authStore.user
  const name = current?.nickname || current?.name || '未知用户'
  const cleanName = name.trim()
  const initials = cleanName
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() ?? '')
      .join('') || 'U'

  return {
    initials,
    name,
    userId: current?.id || '--',
    telegram: cleanName ? `@${cleanName.replace(/^@/, '')}` : '--',
    avatarUrl: current?.avatarUrl || ''
  }
})

const devices = ref<DeviceItem[]>([
])

const projects = ref<ProjectRow[]>([
  {id: 'proj-1', name: 'Yukino Agent', status: '已授权', version: 'v2.3.1', expiresAt: '2026-11-12'},
  {id: 'proj-2', name: 'Yukino Launcher', status: '可下载', version: 'v1.9.8', expiresAt: '2026-08-01'},
  {id: 'proj-3', name: 'Yukino Monitor', status: '内测开放', version: 'v0.9.0-beta', expiresAt: '2026-05-30'}
])

const projectHeaders = [
  {title: '项目名', key: 'name', sortable: false},
  {title: '状态', key: 'status', sortable: false},
  {title: '版本', key: 'version', sortable: false},
  {title: '到期时间', key: 'expiresAt', sortable: false}
]
const unbindLoading = ref(false)
const rebindLoading = ref(false)
const kickLoadingId = ref('')
const bindCode = ref('')
const bindLoading = ref(false)
const activationCode = ref('')
const activationLoading = ref(false)
const projectSearch = ref('')

const filteredProjects = computed(() => {
  const kw = projectSearch.value.trim().toLowerCase()
  if (!kw) return projects.value
  return projects.value.filter((item) => [item.name, item.status, item.version].join(' ').toLowerCase().includes(kw))
})

function handleUnbind() {
  unbindLoading.value = true
  setTimeout(() => {
    unbindLoading.value = false
    feedback.open({type: 'info', message: '解绑请求已提交'})
  }, 600)
}

function handleRebind() {
  rebindLoading.value = true
  setTimeout(() => {
    rebindLoading.value = false
    feedback.open({type: 'success', message: '换绑验证流程已启动'})
  }, 600)
}

function deleteDevice(deviceId: string) {
  kickLoadingId.value = deviceId
  setTimeout(async () => {
    await http.delete('/device/' + deviceId)
    kickLoadingId.value = ''
    feedback.open({type: 'success', message: '设备已下线'})
  }, 500)
}

async function createDevice() {
  if (!bindCode.value.trim()) {
    feedback.open({type: 'error', message: '请输入有效的设备绑定代码'})
    return
  }

  bindLoading.value = true
  try {
    let device_data = atob(bindCode.value.trim()).split(':')
    let data = {
      hardware_id: device_data[0],
      name: device_data[1],
    }
    const device = await http.post('/device', data) as Device

    devices.value = [
      {id: device.hardware_id, name: device.name, lastSeen: '刚刚', icon: 'mdi-laptop'},
      ...devices.value
    ]

    bindCode.value = ''
    feedback.open({type: 'success', message: '设备绑定成功'})
  } finally {
    bindLoading.value = false
  }
}

function submitActivation() {
  if (!activationCode.value.trim()) {
    feedback.open({type: 'error', message: '请输入有效卡密'})
    return
  }

  activationLoading.value = true
  setTimeout(() => {
    activationLoading.value = false
    feedback.open({type: 'success', message: '卡密激活成功（Mock）'})
    activationCode.value = ''
  }, 900)
}
</script>

<style scoped>
.device-list-scroll {
  max-height: 190px;
  overflow-y: auto;
}
</style>
