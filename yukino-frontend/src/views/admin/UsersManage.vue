<template>
  <v-row>
    <v-col cols="12">
      <section class="panel-surface">
        <div class="d-flex flex-column flex-sm-row align-sm-center justify-space-between mb-4 ga-3">
          <h1 class="text-h6 font-weight-bold text-primary">用户管理</h1>
          <v-text-field
              v-model="search"
              class="w-100 search-field"
              clearable
              density="compact"
              hide-details
              placeholder="搜索用户 / 设备"
              prepend-inner-icon="mdi-magnify"
              style="max-width: 360px"
              variant="solo-filled"
          />
        </div>

        <v-skeleton-loader v-if="tableLoading" type="table-thead, table-row@6"/>

        <template v-else>
          <v-data-table :headers="headers" :items="filteredUsers" :items-per-page="10" class="elevation-0" hover
                        item-value="id">
            <template v-slot:[`item.status`]="{ item }">
              <v-chip :color="item.banned ? 'error' : 'success'" size="small" variant="tonal">
                {{ item.banned ? '已封禁' : '正常' }}
              </v-chip>
            </template>

            <template v-slot:[`item.devices`]="{ item }">
              <v-chip color="primary" size="small" variant="tonal">{{ item.onlineDevices }} 台在线</v-chip>
            </template>

            <template v-slot:[`item.maxDevices`]="{ item }">
              <span class="text-medium-emphasis">{{ item.maxDevices }} 台</span>
            </template>

            <template v-slot:[`item.actions`]="{ item }">
              <v-menu location="bottom end">
                <template #activator="{ props }">
                  <v-btn icon="mdi-dots-vertical" v-bind="props" variant="text"/>
                </template>

                <v-card border min-width="190" rounded="lg" variant="flat">
                  <v-list class="py-1" density="compact">
                    <v-list-item prepend-icon="mdi-devices" title="查看在线设备" @click="openDevicesDialog(item.id)"/>
                    <v-list-item prepend-icon="mdi-harddisk-plus" title="调整设备上限"
                                 @click="openDeviceLimitDialog(item.id)"/>
                    <v-list-item prepend-icon="mdi-account-cancel-outline" title="封禁用户"
                                 @click="openBanDialog(item.id)"/>
                  </v-list>
                </v-card>
              </v-menu>
            </template>
          </v-data-table>
        </template>
      </section>
    </v-col>
  </v-row>

  <v-dialog v-model="devicesDialog" max-width="600">
    <v-card class="pa-4 pa-md-5" rounded="xl">
      <div class="d-flex justify-space-between align-center mb-4">
        <h2 class="text-h6 font-weight-bold text-primary">在线设备详情</h2>
        <v-btn icon="mdi-close" variant="text" @click="devicesDialog = false"/>
      </div>

      <v-list v-if="selectedUserDevices.length > 0" class="pa-0 bg-transparent" lines="two">
        <v-list-item
            v-for="device in selectedUserDevices"
            :key="device.id"
            :subtitle="`ID: ${device.id} · 最后活跃: ${device.lastSeen}`"
            :title="device.name"
            class="px-0"
        >
          <template #prepend>
            <v-avatar color="secondary" size="36" variant="tonal">
              <v-icon :icon="device.icon"/>
            </v-avatar>
          </template>
          <template #append>
            <v-btn :loading="kickingDeviceId === device.id" color="error" size="small" variant="text"
                   @click="kickDevice(device.id)">
              下线设备
            </v-btn>
          </template>
        </v-list-item>
      </v-list>

      <v-empty-state v-else icon="mdi-check-circle-outline" text="该用户当前没有在线终端。" title="暂无在线设备"/>
    </v-card>
  </v-dialog>

  <v-dialog v-model="banDialog" max-width="600">
    <v-card class="pa-4 pa-md-5" rounded="xl">
      <div class="d-flex justify-space-between align-center mb-2">
        <h2 class="text-h6 font-weight-bold text-primary">封禁用户</h2>
        <v-btn icon="mdi-close" variant="text" @click="banDialog = false"/>
      </div>

      <p class="text-body-2 text-medium-emphasis mb-4">请选择封禁时长并确认操作。</p>

      <v-radio-group v-model="banDuration" class="mb-2" color="primary">
        <v-radio v-for="option in banOptions" :key="option.value" :label="option.label" :value="option.value"/>
      </v-radio-group>

      <div class="d-flex justify-end ga-2 mt-2">
        <v-btn variant="text" @click="banDialog = false">取消</v-btn>
        <v-btn :loading="banLoading" color="error" @click="submitBan">确认封禁</v-btn>
      </div>
    </v-card>
  </v-dialog>

  <v-dialog v-model="deviceLimitDialog" max-width="600">
    <v-card class="pa-4 pa-md-5" rounded="xl">
      <div class="d-flex justify-space-between align-center mb-2">
        <h2 class="text-h6 font-weight-bold text-primary">调整设备绑定上限</h2>
        <v-btn icon="mdi-close" variant="text" @click="deviceLimitDialog = false"/>
      </div>

      <v-text-field
          v-model.number="editingMaxDevices"
          class="mb-3"
          color="primary"
          density="comfortable"
          label="最大绑定设备数"
          min="1"
          rounded="lg"
          type="number"
          variant="outlined"
      />

      <div class="d-flex justify-end ga-2 mt-2">
        <v-btn variant="text" @click="deviceLimitDialog = false">取消</v-btn>
        <v-btn :loading="limitLoading" color="primary" @click="submitDeviceLimit">保存</v-btn>
      </div>
    </v-card>
  </v-dialog>
</template>

<script lang="ts" setup>
import {computed, ref} from 'vue'
import http from '../../api/axios'
import {useFeedbackStore} from '../../stores/feedback'

interface Device {
  id: string
  name: string
  lastSeen: string
  icon: string
  online: boolean
}

interface UserRecord {
  id: string
  name: string
  role: string
  banned: boolean
  maxDevices: number
  devices: Device[]
}

interface UserRow {
  id: string
  name: string
  role: string
  banned: boolean
  onlineDevices: number
  maxDevices: number
  searchableText: string
}

const USE_MOCK = (import.meta.env.VITE_USE_MOCK ?? 'true') !== 'false'

const feedback = useFeedbackStore()
const tableLoading = ref(true)
const search = ref('')

const devicesDialog = ref(false)
const banDialog = ref(false)
const deviceLimitDialog = ref(false)
const banLoading = ref(false)
const limitLoading = ref(false)
const kickingDeviceId = ref('')

const selectedUserId = ref<string>('')
const banDuration = ref<number>(24)
const editingMaxDevices = ref(3)

const banOptions = [
  {label: '24 小时', value: 24},
  {label: '3 天', value: 72},
  {label: '7 天', value: 168},
  {label: '永久封禁', value: -1}
]

const headers = [
  {title: '用户名', key: 'name', sortable: false},
  {title: '角色', key: 'role', sortable: false},
  {title: '状态', key: 'status', sortable: false},
  {title: '在线设备', key: 'devices', sortable: false},
  {title: '设备上限', key: 'maxDevices', sortable: false},
  {title: '操作', key: 'actions', sortable: false, align: 'end' as const}
]
const users = ref<UserRecord[]>([
  {
    id: 'u-101',
    name: 'Akari',
    role: 'user',
    banned: false,
    maxDevices: 3,
    devices: [
      {id: 'd-1', name: 'Windows Desktop', lastSeen: '刚刚', icon: 'mdi-monitor', online: true},
      {id: 'd-2', name: 'Android Phone', lastSeen: '1 分钟前', icon: 'mdi-cellphone', online: true}
    ]
  },
  {
    id: 'u-102',
    name: 'Mio',
    role: 'user',
    banned: false,
    maxDevices: 2,
    devices: [{id: 'd-3', name: 'MacBook Pro', lastSeen: '4 分钟前', icon: 'mdi-laptop', online: true}]
  },
  {
    id: 'u-103',
    name: 'Sora',
    role: 'user',
    banned: true,
    maxDevices: 1,
    devices: [{id: 'd-4', name: 'Linux Workstation', lastSeen: '12 分钟前', icon: 'mdi-desktop-classic', online: true}]
  }
])

setTimeout(() => {
  tableLoading.value = false
}, 450)

const flattenedUsers = computed<UserRow[]>(() => {
  return users.value.map((user) => {
    const onlineDevices = user.devices.filter((device) => device.online).length
    const deviceNames = user.devices.map((device) => device.name).join(' ')

    return {
      id: user.id,
      name: user.name,
      role: user.role,
      banned: user.banned,
      onlineDevices,
      maxDevices: user.maxDevices,
      searchableText: `${user.name} ${user.role} ${deviceNames}`.toLowerCase()
    }
  })
})

const filteredUsers = computed(() => {
  const keyword = search.value.trim().toLowerCase()
  if (!keyword) {
    return flattenedUsers.value
  }

  return flattenedUsers.value.filter((user) => user.searchableText.includes(keyword))
})

const selectedUser = computed(() => users.value.find((user) => user.id === selectedUserId.value) ?? null)

const selectedUserDevices = computed(() => selectedUser.value?.devices.filter((device) => device.online) ?? [])

function openDevicesDialog(userId: string) {
  selectedUserId.value = userId
  devicesDialog.value = true
}

function openBanDialog(userId: string) {
  selectedUserId.value = userId
  banDuration.value = 24
  banDialog.value = true
}

function openDeviceLimitDialog(userId: string) {
  selectedUserId.value = userId
  editingMaxDevices.value = selectedUser.value?.maxDevices ?? 1
  deviceLimitDialog.value = true
}

async function requestBanUser(userId: string, durationHours: number) {
  if (USE_MOCK) {
    await new Promise((resolve) => setTimeout(resolve, 600))
    return
  }

  await http.post(`/admin/users/${userId}/ban`, {durationHours})
}

async function requestKickDevice(deviceId: string) {
  if (USE_MOCK) {
    await new Promise((resolve) => setTimeout(resolve, 400))
    return
  }

  await http.post(`/admin/devices/${deviceId}/kick`)
}

async function requestUpdateMaxDevices(userId: string, maxDevices: number) {
  if (USE_MOCK) {
    await new Promise((resolve) => setTimeout(resolve, 450))
    return
  }

  await http.post(`/admin/users/${userId}/max-devices`, {maxDevices})
}

async function submitBan() {
  const user = selectedUser.value
  if (!user) {
    return
  }

  banLoading.value = true

  try {
    await requestBanUser(user.id, banDuration.value)

    users.value = users.value.map((record) => (record.id === user.id ? {...record, banned: true} : record))

    feedback.open({type: 'success', message: `${user.name} 已封禁`})
    banDialog.value = false
  } finally {
    banLoading.value = false
  }
}

async function submitDeviceLimit() {
  const user = selectedUser.value
  if (!user) {
    return
  }

  const next = Math.max(1, Math.floor(Number(editingMaxDevices.value) || 1))
  limitLoading.value = true

  try {
    await requestUpdateMaxDevices(user.id, next)

    users.value = users.value.map((record) => (record.id === user.id ? {...record, maxDevices: next} : record))

    feedback.open({type: 'success', message: `${user.name} 设备上限已更新`})
    deviceLimitDialog.value = false
  } finally {
    limitLoading.value = false
  }
}

async function kickDevice(deviceId: string) {
  const user = selectedUser.value
  if (!user) {
    return
  }

  kickingDeviceId.value = deviceId

  try {
    await requestKickDevice(deviceId)

    users.value = users.value.map((record) => {
      if (record.id !== user.id) {
        return record
      }

      return {
        ...record,
        devices: record.devices.map((device) => (device.id === deviceId ? {...device, online: false} : device))
      }
    })

    feedback.open({type: 'success', message: '设备已强制下线'})
  } finally {
    kickingDeviceId.value = ''
  }
}
</script>
