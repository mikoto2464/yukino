<template>
  <!-- 用户管理 -->
  <section class="card-surface">
    <div
      style="
        display: flex;
        align-items: center;
        justify-content: space-between;
        flex-wrap: wrap;
        gap: 12px;
      "
    >
      <h1 class="page-title">用户管理</h1>
      <input
        v-model="search"
        class="input-field"
        placeholder="搜索用户 / 设备"
        style="max-width: 320px"
      />
    </div>

    <!-- 骨架屏 -->
    <div v-if="tableLoading" style="margin-top: 16px">
      <div
        v-for="i in 5"
        :key="i"
        class="skeleton"
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
              <span :class="u.banned ? 'chip chip-error' : 'chip chip-success'">
                {{ u.banned ? "已封禁" : "正常" }}
              </span>
            </td>
            <td>
              <span class="chip chip-info">{{ u.onlineDevices }} 台在线</span>
            </td>
            <td>{{ u.maxDevices }} 台</td>
            <td>
              <!-- 操作菜单按钮 -->
              <div class="action-menu-wrapper">
                <button class="btn btn-text action-trigger">
                  <md-icon>more_vert</md-icon>
                </button>
                <div class="action-menu">
                  <button class="action-menu-item" @click="openDevices(u.id)">
                    查看在线设备
                  </button>
                  <button
                    class="action-menu-item"
                    @click="openDeviceLimit(u.id)"
                  >
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
  </section>

  <!-- ========= 弹窗：在线设备详情 ========= -->
  <Teleport to="body">
    <div
      v-if="devicesDialog"
      class="dialog-overlay"
      @click.self="devicesDialog = false"
    >
      <div class="dialog-card">
        <div class="dialog-header">
          <h2 class="page-title" style="font-size: 1.125rem">在线设备详情</h2>
          <button class="btn btn-text" @click="devicesDialog = false">
            关闭
          </button>
        </div>
        <div v-if="selectedUserDevices.length > 0">
          <div
            v-for="d in selectedUserDevices"
            :key="d.id"
            style="
              display: flex;
              align-items: center;
              gap: 12px;
              padding: 10px 0;
              border-bottom: 1px solid var(--md-sys-color-outline-variant);
            "
          >
            <md-icon style="--md-icon-size: 20px">devices</md-icon>
            <div style="flex: 1">
              <div style="font-weight: 500">{{ d.name }}</div>
              <div
                style="
                  font-size: 0.75rem;
                  color: var(--md-sys-color-on-surface-variant);
                "
              >
                最后活跃: {{ d.lastSeen }}
              </div>
            </div>
            <button
              class="btn btn-text"
              style="color: var(--md-sys-color-error)"
              :disabled="kickingDeviceId === d.id"
              @click="handleKick(d.id)"
            >
              下线设备
            </button>
          </div>
        </div>
        <p
          v-else
          class="section-subtitle"
          style="text-align: center; padding: 24px 0"
        >
          该用户当前没有在线终端。
        </p>
      </div>
    </div>
  </Teleport>

  <!-- ========= 弹窗：封禁用户 ========= -->
  <Teleport to="body">
    <div
      v-if="banDialog"
      class="dialog-overlay"
      @click.self="banDialog = false"
    >
      <div class="dialog-card">
        <div class="dialog-header">
          <h2 class="page-title" style="font-size: 1.125rem">封禁用户</h2>
          <button class="btn btn-text" @click="banDialog = false">关闭</button>
        </div>
        <p class="section-subtitle" style="margin-bottom: 16px">
          请选择封禁时长并确认操作。
        </p>
        <div style="display: flex; flex-direction: column; gap: 8px">
          <label
            v-for="opt in banOptions"
            :key="opt.value"
            style="
              display: flex;
              align-items: center;
              gap: 12px;
              padding: 8px 0;
              cursor: pointer;
            "
          >
            <input
              v-model="banDuration"
              type="radio"
              :value="opt.value"
              style="accent-color: var(--md-sys-color-primary)"
            />
            {{ opt.label }}
          </label>
        </div>
        <div
          style="
            display: flex;
            justify-content: flex-end;
            gap: 8px;
            margin-top: 16px;
          "
        >
          <button class="btn btn-text" @click="banDialog = false">取消</button>
          <button
            class="btn btn-error"
            :disabled="banLoading"
            @click="submitBan"
          >
            确认封禁
          </button>
        </div>
      </div>
    </div>
  </Teleport>

  <!-- ========= 弹窗：调整设备上限 ========= -->
  <Teleport to="body">
    <div
      v-if="deviceLimitDialog"
      class="dialog-overlay"
      @click.self="deviceLimitDialog = false"
    >
      <div class="dialog-card">
        <div class="dialog-header">
          <h2 class="page-title" style="font-size: 1.125rem">
            调整设备绑定上限
          </h2>
          <button class="btn btn-text" @click="deviceLimitDialog = false">
            关闭
          </button>
        </div>
        <input
          v-model.number="editingMaxDevices"
          class="input-field"
          type="number"
          min="1"
          placeholder="最大绑定设备数"
        />
        <div
          style="
            display: flex;
            justify-content: flex-end;
            gap: 8px;
            margin-top: 16px;
          "
        >
          <button class="btn btn-text" @click="deviceLimitDialog = false">
            取消
          </button>
          <button
            class="btn btn-primary"
            :disabled="limitLoading"
            @click="submitDeviceLimit"
          >
            保存
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script lang="ts" setup>
// @material/web icon
import "@material/web/icon/icon.js";
import { computed, ref } from "vue";
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
      {
        id: "d-1",
        name: "Windows Desktop",
        lastSeen: "刚刚",
        icon: "monitor",
        online: true,
      },
      {
        id: "d-2",
        name: "Android Phone",
        lastSeen: "1 分钟前",
        icon: "phone_android",
        online: true,
      },
    ],
  },
  {
    id: "u-102",
    name: "Mio",
    role: "user",
    banned: false,
    maxDevices: 2,
    devices: [
      {
        id: "d-3",
        name: "MacBook Pro",
        lastSeen: "4 分钟前",
        icon: "laptop",
        online: true,
      },
    ],
  },
  {
    id: "u-103",
    name: "Sora",
    role: "user",
    banned: true,
    maxDevices: 1,
    devices: [
      {
        id: "d-4",
        name: "Linux Workstation",
        lastSeen: "12 分钟前",
        icon: "desktop_windows",
        online: true,
      },
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
  const next = Math.max(1, Math.floor(Number(editingMaxDevices.value) || 1));
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
</script>

<style scoped>
/* 操作菜单 */
.action-menu-wrapper {
  position: relative;
}
.action-trigger {
  padding: 4px;
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
  box-shadow: var(--md-sys-elevation-level3, 0 4px 8px 3px rgba(0, 0, 0, 0.15));
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

/* 弹窗覆盖层 */
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.4);
}
.dialog-card {
  width: 100%;
  max-width: 520px;
  max-height: 80vh;
  overflow-y: auto;
  padding: 24px;
  border-radius: 24px;
  background: var(--md-sys-color-surface-container-high, #ece6f0);
  box-shadow: var(--md-sys-elevation-level5, 0 8px 16px rgba(0, 0, 0, 0.2));
}
.dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}
</style>
