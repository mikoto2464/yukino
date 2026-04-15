<template>
  <v-row>
    <v-col cols="12">
      <section class="panel-surface mb-6">
        <div class="d-flex align-center justify-space-between mb-4">
          <h1 class="text-h6 font-weight-bold text-primary">卡密生成</h1>
          <v-btn color="primary" variant="tonal" @click="addConfigRow">新增项目配置</v-btn>
        </div>

        <v-row v-for="row in formRows" :key="row.id" class="mb-1">
          <v-col cols="12" lg="6" md="7">
            <v-autocomplete
                v-model="row.projectId"
                :items="projectOptions"
                clearable
                color="primary"
                density="comfortable"
                hide-details="auto"
                item-title="name"
                item-value="id"
                label="选择项目"
                rounded="lg"
                variant="outlined"
            />
          </v-col>

          <v-col cols="10" lg="5" md="4">
            <v-select
                v-model="row.period"
                :items="periodOptions"
                color="primary"
                density="comfortable"
                hide-details="auto"
                label="授权周期"
                rounded="lg"
                variant="outlined"
            />
          </v-col>

          <v-col class="d-flex align-center justify-end" cols="2" lg="1" md="1">
            <v-btn
                :disabled="formRows.length === 1"
                color="error"
                icon="mdi-delete-outline"
                variant="text"
                @click="removeConfigRow(row.id)"
            />
          </v-col>
        </v-row>

        <v-btn :loading="generateLoading" class="mt-4" color="primary" size="large" @click="generateKeys">生成卡密
        </v-btn>
      </section>
    </v-col>

    <v-col cols="12">
      <section class="panel-surface">
        <div class="d-flex flex-column flex-sm-row align-sm-center justify-space-between mb-4 ga-3">
          <h2 class="text-h6 font-weight-bold text-primary">卡密管理</h2>
          <v-text-field
              v-model="search"
              class="w-100 search-field"
              clearable
              density="compact"
              hide-details
              placeholder="搜索卡密 / 项目 / 操作人"
              prepend-inner-icon="mdi-magnify"
              style="max-width: 360px"
              variant="solo-filled"
          />
        </div>

        <v-skeleton-loader v-if="tableLoading" type="table-thead, table-row@6"/>

        <template v-else>
          <v-data-table :headers="headers" :items="pagedItems" :items-per-page="itemsPerPage" class="elevation-0"
                        hide-default-footer>
            <template v-slot:[`item.bindings`]="{ item }">
              <div>
                <v-chip
                    v-for="binding in item.bindings"
                    :key="`${item.id}-${binding.projectId}-${binding.period}`"
                    class="mr-1 mb-1"
                    color="primary"
                    size="small"
                    variant="tonal"
                >
                  {{ binding.projectName }} · {{ binding.period }}
                </v-chip>
              </div>
            </template>

            <template v-slot:[`item.status`]="{ item }">
              <v-chip :color="statusMeta[item.status].color" size="small" variant="tonal">
                {{ statusMeta[item.status].label }}
              </v-chip>
            </template>

            <template v-slot:[`item.createdAt`]="{ item }">
              <span class="text-medium-emphasis">{{ item.createdAt }}</span>
            </template>
          </v-data-table>

          <div class="d-flex justify-space-between align-center mt-4 flex-wrap ga-3">
            <div class="text-body-2 text-medium-emphasis">共 {{ filteredItems.length }} 条</div>
            <v-pagination v-model="page" :length="pageCount" density="comfortable" rounded="circle"/>
          </div>
        </template>
      </section>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import {computed, ref, watch} from 'vue'
import {useFeedbackStore} from '../../stores/feedback'

interface ProjectOption {
  id: string
  name: string
}

type PeriodOption = '7天' | '30天' | '90天' | '180天' | '365天'
type KeyStatus = 'active' | 'used' | 'expired'

interface ConfigRow {
  id: string
  projectId: string | null
  period: PeriodOption
}

interface CDKeyBinding {
  projectId: string
  projectName: string
  period: PeriodOption
}

interface CDKeyItem {
  id: string
  key: string
  bindings: CDKeyBinding[]
  status: KeyStatus
  operator: string
  createdAt: string
}

const feedback = useFeedbackStore()

const projectOptions: ProjectOption[] = [
  {id: 'p-agent', name: 'Yukino Agent'},
  {id: 'p-launcher', name: 'Yukino Launcher'},
  {id: 'p-monitor', name: 'Yukino Monitor'},
  {id: 'p-console', name: 'Yukino Console'},
  {id: 'p-mobile', name: 'Yukino Mobile Bridge'}
]

const projectNameMap = projectOptions.reduce<Record<string, string>>((acc, item) => {
  acc[item.id] = item.name
  return acc
}, {})

const periodOptions: PeriodOption[] = ['7天', '30天', '90天', '180天', '365天']

const statusMeta: Record<KeyStatus, { label: string; color: string }> = {
  active: {label: '可用', color: 'success'},
  used: {label: '已使用', color: 'info'},
  expired: {label: '已过期', color: 'error'}
}

const formRows = ref<ConfigRow[]>([
  {
    id: crypto.randomUUID(),
    projectId: projectOptions[0].id,
    period: '30天'
  }
])

const generateLoading = ref(false)
const search = ref('')
const tableLoading = ref(true)
const page = ref(1)
const itemsPerPage = 8

const headers = [
  {title: '卡密', key: 'key', sortable: false},
  {title: '项目周期', key: 'bindings', sortable: false},
  {title: '状态', key: 'status', sortable: false},
  {title: '创建人', key: 'operator', sortable: false},
  {title: '创建时间', key: 'createdAt', sortable: false}
]
const keyItems = ref<CDKeyItem[]>([
  {
    id: 'k-1',
    key: 'YKN-7A3B-91KC-5PQM',
    bindings: [{projectId: 'p-agent', projectName: 'Yukino Agent', period: '30天'}],
    status: 'active',
    operator: 'admin_01',
    createdAt: '2026-04-15 09:12'
  },
  {
    id: 'k-2',
    key: 'YKN-3MX9-8QQE-1NTR',
    bindings: [{projectId: 'p-launcher', projectName: 'Yukino Launcher', period: '90天'}],
    status: 'used',
    operator: 'admin_01',
    createdAt: '2026-04-15 09:35'
  },
  {
    id: 'k-3',
    key: 'YKN-Z11P-6HRT-2BVA',
    bindings: [
      {projectId: 'p-monitor', projectName: 'Yukino Monitor', period: '30天'},
      {projectId: 'p-console', projectName: 'Yukino Console', period: '30天'}
    ],
    status: 'active',
    operator: 'ops_02',
    createdAt: '2026-04-15 10:20'
  }
])

setTimeout(() => {
  tableLoading.value = false
}, 500)

const filteredItems = computed(() => {
  const kw = search.value.trim().toLowerCase()

  if (!kw) {
    return keyItems.value
  }

  return keyItems.value.filter((item) => {
    const bindingText = item.bindings.map((binding) => `${binding.projectName} ${binding.period}`).join(' ')
    return [item.key, item.operator, item.createdAt, bindingText].join(' ').toLowerCase().includes(kw)
  })
})

const pageCount = computed(() => Math.max(1, Math.ceil(filteredItems.value.length / itemsPerPage)))

const pagedItems = computed(() => {
  const start = (page.value - 1) * itemsPerPage
  return filteredItems.value.slice(start, start + itemsPerPage)
})

watch(filteredItems, () => {
  if (page.value > pageCount.value) {
    page.value = 1
  }
})

function addConfigRow() {
  formRows.value.push({
    id: crypto.randomUUID(),
    projectId: null,
    period: '30天'
  })
}

function removeConfigRow(id: string) {
  if (formRows.value.length === 1) {
    return
  }

  formRows.value = formRows.value.filter((row) => row.id !== id)
}

function randomToken() {
  const chars = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789'
  const segment = () => Array.from({length: 4}, () => chars[Math.floor(Math.random() * chars.length)]).join('')
  return `YKN-${segment()}-${segment()}-${segment()}`
}

function generateKeys() {
  const validRows = formRows.value.filter((row) => row.projectId)

  if (validRows.length === 0) {
    feedback.open({type: 'error', message: '请至少配置一个有效项目'})
    return
  }

  generateLoading.value = true

  setTimeout(() => {
    const newRows: CDKeyItem[] = validRows.map((row) => {
      const projectId = row.projectId as string
      return {
        id: crypto.randomUUID(),
        key: randomToken(),
        bindings: [
          {
            projectId,
            projectName: projectNameMap[projectId],
            period: row.period
          }
        ],
        status: 'active',
        operator: 'admin_01',
        createdAt: new Date().toLocaleString('zh-CN', {hour12: false})
      }
    })

    keyItems.value = [...newRows, ...keyItems.value]
    generateLoading.value = false
    page.value = 1

    feedback.open({type: 'success', message: `已生成 ${newRows.length} 条卡密（Mock）`})
  }, 800)
}
</script>
