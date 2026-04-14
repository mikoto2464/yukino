<template>
  <v-row>
    <v-col cols="12">
      <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5">
        <div class="d-flex flex-column flex-sm-row align-sm-center justify-space-between mb-4 ga-3">
          <h1 class="text-h6 font-weight-bold text-primary">用户与设备控制中心</h1>
          <v-text-field
            v-model="search"
            label="搜索用户 / 邮箱 / 设备"
            variant="solo-filled"
            density="compact"
            prepend-inner-icon="mdi-magnify"
            hide-details
            clearable
            class="w-100"
            style="max-width: 360px"
          />
        </div>

        <v-skeleton-loader v-if="tableLoading" type="table-thead, table-row@6" />

        <template v-else>
          <v-data-table
            hover
            :headers="headers"
            :items="filteredUsers"
            item-value="id"
            :items-per-page="10"
            class="elevation-0"
          >
            <template #item.status="{ item }">
              <v-chip :color="item.banned ? 'error' : 'success'" size="small" variant="tonal">
                {{ item.banned ? '已封禁' : '正常' }}
              </v-chip>
            </template>

            <template #item.devices="{ item }">
              <v-chip color="primary" size="small" variant="tonal">
                {{ item.onlineDevices }} 台在线
              </v-chip>
            </template>

            <template #item.actions="{ item }">
              <v-menu location="bottom end">
                <template #activator="{ props }">
                  <v-btn v-bind="props" icon="mdi-dots-vertical" variant="text" />
                </template>

                <v-card rounded="lg" variant="flat" border min-width="180">
                  <v-list density="compact" class="py-1">
                    <v-list-item prepend-icon="mdi-devices" title="查看在线设备" @click="openDevicesDialog(item.id)" />
                    <v-list-item prepend-icon="mdi-account-cancel-outline" title="封禁用户" @click="openBanDialog(item.id)" />
                  </v-list>
                </v-card>
              </v-menu>
            </template>
          </v-data-table>
        </template>
      </v-card>
    </v-col>
  </v-row>

  <v-dialog v-model="devicesDialog" max-width="600">
    <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5">
      <div class="d-flex justify-space-between align-center mb-4">
        <h2 class="text-h6 font-weight-bold text-primary">在线设备详情</h2>
        <v-btn icon="mdi-close" variant="text" @click="devicesDialog = false" />
      </div>

      <v-list lines="two" class="pa-0 bg-transparent" v-if="selectedUserDevices.length > 0">
        <v-list-item
          v-for="device in selectedUserDevices"
          :key="device.id"
          :title="device.name"
          :subtitle="`IP: ${device.ip} · 最后活跃: ${device.lastSeen}`"
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
              :loading="kickingDeviceId === device.id"
              @click="kickDevice(device.id)"
            >
              下线设备
            </v-btn>
          </template>
        </v-list-item>
      </v-list>

      <v-empty-state
        v-else
        icon="mdi-check-circle-outline"
        title="暂无在线设备"
        text="该用户当前没有在线终端。"
      />
    </v-card>
  </v-dialog>

  <v-dialog v-model="banDialog" max-width="600">
    <v-card rounded="xl" elevation="2" class="pa-4 pa-md-5">
      <div class="d-flex justify-space-between align-center mb-2">
        <h2 class="text-h6 font-weight-bold text-primary">封禁用户</h2>
        <v-btn icon="mdi-close" variant="text" @click="banDialog = false" />
      </div>

      <p class="text-body-2 text-medium-emphasis mb-4">请选择封禁时长并确认操作。</p>

      <v-radio-group v-model="banDuration" color="primary" class="mb-2">
        <v-radio v-for="option in banOptions" :key="option.value" :label="option.label" :value="option.value" />
      </v-radio-group>

      <div class="d-flex justify-end ga-2 mt-2">
        <v-btn variant="text" @click="banDialog = false">取消</v-btn>
        <v-btn color="error" :loading="banLoading" @click="submitBan">确认封禁</v-btn>
      </div>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import http from '../../api/axios'
import { useFeedbackStore } from '../../stores/feedback'

interface Device {
  id: string
  name: string
  ip: string
  lastSeen: string
  icon: string
  online: boolean
}

interface UserRecord {
  id: string
  name: string
  email: string
  role: string
  banned: boolean
  devices: Device[]
}

interface UserRow {
  id: string
  name: string
  email: string
  role: string
  banned: boolean
  onlineDevices: number
  searchableText: string
}

const USE_MOCK = (import.meta.env.VITE_USE_MOCK ?? 'true') !== 'false'

const feedback = useFeedbackStore()
const tableLoading = ref(true)
const search = ref('')

const devicesDialog = ref(false)
const banDialog = ref(false)
const banLoading = ref(false)
const kickingDeviceId = ref('')

const selectedUserId = ref<string>('')
const banDuration = ref<number>(24)

const banOptions = [
  { label: '24 小时', value: 24 },
  { label: '3 天', value: 72 },
  { label: '7 天', value: 168 },
  { label: '永久封禁', value: -1 }
]

const headers = [
  { title: '用户名', key: 'name', sortable: false },
  { title: '邮箱', key: 'email', sortable: false },
  { title: '角色', key: 'role', sortable: false },
  { title: '状态', key: 'status', sortable: false },
  { title: '在线设备', key: 'devices', sortable: false },
  { title: '操作', key: 'actions', sortable: false, align: 'end' as const }
]

const users = ref<UserRecord[]>([
  {
    id: 'u-101',
    name: 'Akari',
    email: 'akari@yukino.app',
    role: 'user',
    banned: false,
    devices: [
      { id: 'd-1', name: 'Windows Desktop', ip: '10.0.1.12', lastSeen: '刚刚', icon: 'mdi-monitor', online: true },
      { id: 'd-2', name: 'Android Phone', ip: '10.0.1.88', lastSeen: '1 分钟前', icon: 'mdi-cellphone', online: true }
    ]
  },
  {
    id: 'u-102',
    name: 'Mio',
    email: 'mio@yukino.app',
    role: 'user',
    banned: false,
    devices: [{ id: 'd-3', name: 'MacBook Pro', ip: '10.0.2.16', lastSeen: '4 分钟前', icon: 'mdi-laptop', online: true }]
  },
  {
    id: 'u-103',
    name: 'Sora',
    email: 'sora@yukino.app',
    role: 'user',
    banned: true,
    devices: [{ id: 'd-4', name: 'Linux Workstation', ip: '10.0.3.7', lastSeen: '12 分钟前', icon: 'mdi-desktop-classic', online: true }]
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
      email: user.email,
      role: user.role,
      banned: user.banned,
      onlineDevices,
      searchableText: `${user.name} ${user.email} ${user.role} ${deviceNames}`.toLowerCase()
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

const selectedUserDevices = computed(() => {
  const record = selectedUser.value
  if (!record) {
    return []
  }

  return record.devices.filter((device) => device.online)
})

function openDevicesDialog(userId: string) {
  selectedUserId.value = userId
  devicesDialog.value = true
}

function openBanDialog(userId: string) {
  selectedUserId.value = userId
  banDuration.value = 24
  banDialog.value = true
}

async function requestBanUser(userId: string, durationHours: number) {
  if (USE_MOCK) {
    await new Promise((resolve) => setTimeout(resolve, 600))
    return
  }

  await http.post(`/admin/users/${userId}/ban`, { durationHours })
}

async function requestKickDevice(deviceId: string) {
  if (USE_MOCK) {
    await new Promise((resolve) => setTimeout(resolve, 400))
    return
  }

  await http.post(`/admin/devices/${deviceId}/kick`)
}

async function submitBan() {
  const user = selectedUser.value
  if (!user) {
    return
  }

  banLoading.value = true

  try {
    await requestBanUser(user.id, banDuration.value)

    users.value = users.value.map((record) => {
      if (record.id !== user.id) {
        return record
      }

      return { ...record, banned: true }
    })

    feedback.open({
      type: 'success',
      message: `${user.name} 已封禁`
    })

    banDialog.value = false
  } finally {
    banLoading.value = false
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
        devices: record.devices.map((device) => {
          if (device.id !== deviceId) {
            return device
          }

          return { ...device, online: false }
        })
      }
    })

    feedback.open({
      type: 'success',
      message: '设备已强制下线'
    })
  } finally {
    kickingDeviceId.value = ''
  }
}
</script>
