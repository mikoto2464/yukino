<template>
  <MdCard>
    <div
      style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:12px"
    >
      <h1 style="font: var(--md-sys-typescale-headline-small)">用户管理</h1>
      <MdTextField
        v-model="search"
        placeholder="搜索用户 / 设备"
        style="max-width: 320px"
      />
    </div>

    <!-- 骨架屏 -->
    <div v-if="tableLoading" style="margin-top: 16px">
      <div
        v-for="i in 5"
        :key="i"
        class="md-skeleton"
        style="height: 48px; margin-bottom: 8px"
      />
    </div>

    <!-- 表格 -->
    <template v-else>
      <table class="data-table" style="margin-top: 12px">
        <thead>
          <tr>
            <th>用户名</th>
            <th>角色</th>
            <th>状态</th>
            <th>在线设备</th>
            <th>设备上限</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="u in filteredUsers" :key="u.id">
            <td>{{ u.name }}</td>
            <td>{{ u.role }}</td>
            <td>
              <MdChip :variant="u.banned ? 'filter' : 'assist'">
                {{ u.banned ? "已封禁" : "正常" }}
              </MdChip>
            </td>
            <td>
              <MdChip variant="assist">{{ u.onlineDevices }} 台在线</MdChip>
            </td>
            <td>{{ u.maxDevices }} 台</td>
            <td>
              <div class="action-menu-wrapper">
                <MdIconButton icon="more_vert" :icon-size="20" />
                <div class="action-menu">
                  <button class="action-menu-item" @click="openDevices(u.id)">
                    查看在线设备
                  </button>
                  <button class="action-menu-item" @click="openDeviceLimit(u.id)">
                    调整设备上限
                  </button>
                  <button
                    class="action-menu-item"
                    style="color: var(--md-sys-color-error)"
                    @click="openBan(u.id)"
                  >
                    封禁用户
                  </button>
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </template>
  </MdCard>

  <!-- 弹窗：在线设备详情 -->
  <MdDialog :open="devicesDialog" :headline="'在线设备详情'" @update:open="devicesDialog = $event">
    <div v-if="selectedUserDevices.length > 0">
      <div
        v-for="d in selectedUserDevices"
        :key="d.id"
        style="display:flex;align-items:center;gap:12px;padding:10px 0;border-bottom:1px solid var(--md-sys-color-outline-variant)"
      >
        <MdIcon icon="devices" :size="20" />
        <div style="flex: 1">
          <div style="font-weight: 500">{{ d.name }}</div>
          <div style="font-size: 0.75rem; color: var(--md-sys-color-on-surface-variant)">
            最后活跃: {{ d.lastSeen }}
          </div>
        </div>
        <MdButton
          variant="text"
          style="color: var(--md-sys-color-error, #b3261e)"
          :disabled="kickingDeviceId === d.id"
          @click="handleKick(d.id)"
        >
          下线设备
        </MdButton>
      </div>
    </div>
    <p v-else style="text-align:center;padding:24px 0;color:var(--md-sys-color-on-surface-variant)">
      该用户当前没有在线终端。
    </p>
    <template #actions>
      <MdButton variant="text" @click="devicesDialog = false">关闭</MdButton>
    </template>
  </MdDialog>

  <!-- 弹窗：封禁用户 -->
  <MdDialog :open="banDialog" :headline="'封禁用户'" @update:open="banDialog = $event">
    <p style="font: var(--md-sys-typescale-body-medium); color: var(--md-sys-color-on-surface-variant); margin-bottom: 16px">
      请选择封禁时长并确认操作。
    </p>
    <div style="display:flex;flex-direction:column;gap:8px">
      <label
        v-for="opt in banOptions"
        :key="opt.value"
        style="display:flex;align-items:center;gap:12px;padding:8px 0;cursor:pointer"
      >
        <input
          v-model="banDuration"
          type="radio"
          :value="opt.value"
          class="md-radio"
        />
        {{ opt.label }}
      </label>
    </div>
    <template #actions>
      <MdButton variant="text" @click="banDialog = false">取消</MdButton>
      <MdButton
        variant="filled"
        style="background: var(--md-sys-color-error, #b3261e); color: var(--md-sys-color-on-error, #fff)"
        :disabled="banLoading"
        @click="submitBan"
      >
        确认封禁
      </MdButton>
    </template>
  </MdDialog>

  <!-- 弹窗：调整设备上限 -->
  <MdDialog :open="deviceLimitDialog" :headline="'调整设备绑定上限'" @update:open="deviceLimitDialog = $event">
    <MdTextField
      v-model="editingMaxDevicesStr"
      type="number"
      placeholder="最大绑定设备数"
    />
    <template #actions>
      <MdButton variant="text" @click="deviceLimitDialog = false">取消</MdButton>
      <MdButton variant="filled" :disabled="limitLoading" @click="submitDeviceLimit">
        保存
      </MdButton>
    </template>
  </MdDialog>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import MdCard from "@/components/md/MdCard.vue";
import MdButton from "@/components/md/MdButton.vue";
import MdTextField from "@/components/md/MdTextField.vue";
import MdChip from "@/components/md/MdChip.vue";
import MdIcon from "@/components/md/MdIcon.vue";
import MdIconButton from "@/components/md/MdIconButton.vue";
import MdDialog from "@/components/md/MdDialog.vue";
import { useFeedbackStore } from "@/stores/feedback";
import { banUser, kickDevice, updateUserMaxDevices } from "@/api/services";

// ---------- 类型 ----------
interface DeviceInfo {
  id: string;
  name: string;
  lastSeen: string;
  icon: string;
  online: boolean;
}

interface UserRecord {
  id: string;
  name: string;
  role: string;
  banned: boolean;
  maxDevices: number;
  devices: DeviceInfo[];
}

interface UserRow {
  id: string;
  name: string;
  role: string;
  banned: boolean;
  onlineDevices: number;
  maxDevices: number;
  searchableText: string;
}

const feedback = useFeedbackStore();

// ---------- 列表数据 ----------
const tableLoading = ref(true);
const search = ref("");

const users = ref<UserRecord[]>([
  {
    id: "u-101",
    name: "Akari",
    role: "user",
    banned: false,
    maxDevices: 3,
    devices: [
      { id: "d-1", name: "Windows Desktop", lastSeen: "刚刚", icon: "monitor", online: true },
      { id: "d-2", name: "Android Phone", lastSeen: "1 分钟前", icon: "phone_android", online: true },
    ],
  },
  {
    id: "u-102",
    name: "Mio",
    role: "user",
    banned: false,
    maxDevices: 2,
    devices: [
      { id: "d-3", name: "MacBook Pro", lastSeen: "4 分钟前", icon: "laptop", online: true },
    ],
  },
  {
    id: "u-103",
    name: "Sora",
    role: "user",
    banned: true,
    maxDevices: 1,
    devices: [
      { id: "d-4", name: "Linux Workstation", lastSeen: "12 分钟前", icon: "desktop_windows", online: true },
    ],
  },
]);

setTimeout(() => {
  tableLoading.value = false;
}, 450);

const flattenedUsers = computed<UserRow[]>(() =>
  users.value.map((u) => ({
    id: u.id,
    name: u.name,
    role: u.role,
    banned: u.banned,
    onlineDevices: u.devices.filter((d) => d.online).length,
    maxDevices: u.maxDevices,
    searchableText:
      `${u.name} ${u.role} ${u.devices.map((d) => d.name).join(" ")}`.toLowerCase(),
  })),
);

const filteredUsers = computed(() => {
  const kw = search.value.trim().toLowerCase();
  return kw
    ? flattenedUsers.value.filter((u) => u.searchableText.includes(kw))
    : flattenedUsers.value;
});

// ---------- 弹窗状态 ----------
const devicesDialog = ref(false);
const banDialog = ref(false);
const deviceLimitDialog = ref(false);
const banLoading = ref(false);
const limitLoading = ref(false);
const kickingDeviceId = ref("");
const selectedUserId = ref("");
const banDuration = ref(24);
const editingMaxDevices = ref(3);
const editingMaxDevicesStr = ref("3");

const banOptions = [
  { label: "24 小时", value: 24 },
  { label: "3 天", value: 72 },
  { label: "7 天", value: 168 },
  { label: "永久封禁", value: -1 },
];

const selectedUser = computed<UserRecord | undefined>(() =>
  users.value.find((u) => u.id === selectedUserId.value),
);

const selectedUserDevices = computed<DeviceInfo[]>(
  () => selectedUser.value?.devices.filter((d) => d.online) ?? [],
);

// ---------- 打开弹窗 ----------
function openDevices(userId: string) {
  selectedUserId.value = userId;
  devicesDialog.value = true;
}

function openBan(userId: string) {
  selectedUserId.value = userId;
  banDuration.value = 24;
  banDialog.value = true;
}

function openDeviceLimit(userId: string) {
  selectedUserId.value = userId;
  editingMaxDevices.value = selectedUser.value?.maxDevices ?? 1;
  editingMaxDevicesStr.value = String(editingMaxDevices.value);
  deviceLimitDialog.value = true;
}

// ---------- 操作 ----------
async function submitBan() {
  const u = selectedUser.value;
  if (!u) return;
  banLoading.value = true;
  try {
    await banUser(u.id, banDuration.value);
    users.value = users.value.map((r) =>
      r.id === u.id ? { ...r, banned: true } : r,
    );
    feedback.open({ type: "success", message: `${u.name} 已封禁` });
    banDialog.value = false;
  } finally {
    banLoading.value = false;
  }
}

async function submitDeviceLimit() {
  const u = selectedUser.value;
  if (!u) return;
  const next = Math.max(1, Math.floor(Number(editingMaxDevicesStr.value) || 1));
  limitLoading.value = true;
  try {
    await updateUserMaxDevices(u.id, next);
    users.value = users.value.map((r) =>
      r.id === u.id ? { ...r, maxDevices: next } : r,
    );
    feedback.open({ type: "success", message: `${u.name} 设备上限已更新` });
    deviceLimitDialog.value = false;
  } finally {
    limitLoading.value = false;
  }
}

async function handleKick(deviceId: string) {
  const u = selectedUser.value;
  if (!u) return;
  kickingDeviceId.value = deviceId;
  try {
    await kickDevice(deviceId);
    users.value = users.value.map((r) =>
      r.id === u.id
        ? {
            ...r,
            devices: r.devices.map((d) =>
              d.id === deviceId ? { ...d, online: false } : d,
            ),
          }
        : r,
    );
    feedback.open({ type: "success", message: "设备已强制下线" });
  } finally {
    kickingDeviceId.value = "";
  }
}

// ---- Watch for editingMaxDevices sync ----
watch(editingMaxDevicesStr, (val) => {
  const n = Number(val);
  if (!isNaN(n) && n > 0) editingMaxDevices.value = n;
});
</script>

<style scoped>
.data-table {
  width: 100%;
  border-collapse: collapse;
}
.data-table th,
.data-table td {
  text-align: left;
  padding: 12px 16px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant, #cac4d0);
}
.data-table th {
  font-weight: 500;
  font-size: 0.75rem;
  letter-spacing: 0.025em;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}
.data-table td {
  font-size: 0.875rem;
}

.action-menu-wrapper {
  position: relative;
}
.action-menu-wrapper:hover .action-menu,
.action-menu-wrapper:focus-within .action-menu {
  display: flex;
}
.action-menu {
  display: none;
  flex-direction: column;
  position: absolute;
  right: 0;
  top: 100%;
  min-width: 160px;
  padding: 4px 0;
  border-radius: 12px;
  background: var(--md-sys-color-surface-container, #f3edf7);
  box-shadow: var(--md-sys-elevation-level3);
  z-index: 100;
}
.action-menu-item {
  display: block;
  width: 100%;
  text-align: left;
  padding: 10px 16px;
  border: none;
  background: transparent;
  font-size: 0.8125rem;
  color: var(--md-sys-color-on-surface, #1d1b20);
  cursor: pointer;
}
.action-menu-item:hover {
  background: var(--md-sys-color-surface-container-high, #ece6f0);
}

.md-radio {
  accent-color: var(--md-sys-color-primary, #6750a4);
  width: 18px;
  height: 18px;
}
</style>
