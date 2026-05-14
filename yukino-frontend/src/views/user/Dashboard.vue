<template>
  <div class="dashboard">
    <!-- 第一行：用户信息 + 设备管理 -->
    <div class="dash-grid-2col">
      <!-- 用户信息卡片 -->
      <MdCard>
        <h2 style="font: var(--md-sys-typescale-title-medium)">用户信息</h2>

        <div class="user-profile">
          <div class="avatar">
            <img
              v-if="userInfo.avatarUrl"
              :src="userInfo.avatarUrl"
              alt="头像"
            />
            <span v-else class="avatar-fallback">{{ userInfo.initials }}</span>
          </div>
          <div class="user-meta">
            <div class="user-name">{{ userInfo.name }}</div>
            <div class="user-detail">用户编号: {{ userInfo.id }}</div>
            <div class="user-detail">
              绑定 Telegram: {{ userInfo.telegram }}
            </div>
          </div>
        </div>

      </MdCard>

      <!-- 设备管理卡片 -->
      <MdCard>
        <div class="device-header">
          <h2 style="font: var(--md-sys-typescale-title-medium)">设备列表</h2>
          <div class="device-add">
            <MdTextField
              v-model="bindCode"
              placeholder="输入绑定代码"
              style="width: 240px"
            />
            <MdButton variant="filled" :disabled="bindLoading" @click="handleCreateDevice">
              新增设备
            </MdButton>
          </div>
        </div>

        <div v-if="devices.length > 0" class="device-list">
          <div v-for="d in devices" :key="d.hardware_id" class="device-item">
            <div class="device-icon-wrapper">
              <MdIcon icon="devices" :size="20" />
            </div>
            <div class="device-info">
              <div class="device-name">{{ d.name }}</div>
              <div class="device-sub">
                最后心跳: {{ formatTime(d.last_seen) }}
              </div>
            </div>
            <MdButton
              variant="text"
              style="color: var(--md-sys-color-error, #b3261e)"
              :disabled="kickingId === d.hardware_id"
              @click="handleDeleteDevice(d.hardware_id)"
            >
              下线
            </MdButton>
          </div>
        </div>
        <p v-else style="font: var(--md-sys-typescale-body-small); color: var(--md-sys-color-on-surface-variant)">暂无绑定设备</p>
      </MdCard>
    </div>

    <!-- 第二行：项目管理 + 卡密激活 -->
    <MdCard style="margin-top: var(--md-spacing-md, 24px)">
      <div class="projects-header">
        <h2 style="font: var(--md-sys-typescale-title-medium)">可用项目</h2>
        <div class="activation-row">
          <MdTextField
            v-model="activationCode"
            placeholder="输入卡密"
            style="width: 240px"
          />
          <MdButton variant="filled" :disabled="activationLoading" @click="handleActivation">
            激活卡密
          </MdButton>
        </div>
      </div>

      <table class="data-table" style="margin-top: var(--md-spacing-md, 24px)">
        <thead>
          <tr>
            <th>项目名</th>
            <th>状态</th>
            <th>版本</th>
            <th>到期时间</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="p in projects" :key="p.id">
            <td>{{ p.name }}</td>
            <td>
              <MdChip variant="assist">{{ p.status }}</MdChip>
            </td>
            <td>{{ p.version }}</td>
            <td>{{ p.expiresAt }}</td>
          </tr>
        </tbody>
      </table>
    </MdCard>
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from "vue";
import MdCard from "@/components/md/MdCard.vue";
import MdButton from "@/components/md/MdButton.vue";
import MdTextField from "@/components/md/MdTextField.vue";
import MdChip from "@/components/md/MdChip.vue";
import MdIcon from "@/components/md/MdIcon.vue";
import { useAuthStore } from "@/stores/auth";
import { useFeedbackStore } from "@/stores/feedback";
import {
  fetchDevices,
  createDevice,
  deleteDevice,
  redeemCdkey,
} from "@/api/services";
import type { Device } from "@/types";

const auth = useAuthStore();
const feedback = useFeedbackStore();

// ---------- 用户信息 ----------
const userInfo = computed(() => {
  const u = auth.user;
  const name = u?.nickname || u?.name || "未知用户";
  const initials =
    name
      .split(/\s+/)
      .filter(Boolean)
      .slice(0, 2)
      .map((p) => p[0]?.toUpperCase() ?? "")
      .join("") || "U";

  return {
    initials,
    name,
    id: u?.id || "--",
    telegram: name ? `@${name.replace(/^@/, "")}` : "--",
    avatarUrl: u?.avatarUrl || "",
  };
});

// ---------- 设备管理 ----------
const devices = ref<Device[]>([]);
const bindCode = ref("");
const bindLoading = ref(false);
const kickingId = ref("");


async function loadDevices() {
  try {
    devices.value = await fetchDevices();
  } catch {
    // 静默失败
  }
}

function formatTime(ts: number): string {
  const d = new Date(ts * 1000);
  return d.toLocaleString("zh-CN", { hour12: false });
}

async function handleCreateDevice() {
  const raw = bindCode.value.trim();
  if (!raw) {
    feedback.open({ type: "error", message: "请输入有效的设备绑定代码" });
    return;
  }
  bindLoading.value = true;
  try {
    const decoded = atob(raw).split(":");
    const payload = {
      hardware_id: decoded[0] ?? raw,
      name: decoded[1] ?? "未命名设备",
    };
    const device = await createDevice(payload);
    devices.value = [device, ...devices.value];
    bindCode.value = "";
    feedback.open({ type: "success", message: "设备绑定成功" });
  } catch {
    feedback.open({ type: "error", message: "绑定失败，请检查代码格式" });
  } finally {
    bindLoading.value = false;
  }
}

async function handleDeleteDevice(hardwareId: string) {
  kickingId.value = hardwareId;
  try {
    await deleteDevice(hardwareId);
    devices.value = devices.value.filter((d) => d.hardware_id !== hardwareId);
    feedback.open({ type: "success", message: "设备已下线" });
  } finally {
    kickingId.value = "";
  }
}

// ---------- 项目与卡密 ----------
interface ProjectRow {
  id: string;
  name: string;
  status: string;
  version: string;
  expiresAt: string;
}

const projects = ref<ProjectRow[]>([
  {
    id: "1",
    name: "Yukino Agent",
    status: "已授权",
    version: "v2.3.1",
    expiresAt: "2026-11-12",
  },
  {
    id: "2",
    name: "Yukino Launcher",
    status: "可下载",
    version: "v1.9.8",
    expiresAt: "2026-08-01",
  },
  {
    id: "3",
    name: "Yukino Monitor",
    status: "内测开放",
    version: "v0.9.0-beta",
    expiresAt: "2026-05-30",
  },
]);

const activationCode = ref("");
const activationLoading = ref(false);

async function handleActivation() {
  const code = activationCode.value.trim();
  if (!code) {
    feedback.open({ type: "error", message: "请输入有效卡密" });
    return;
  }
  activationLoading.value = true;
  try {
    await redeemCdkey({ cdkey: code });
    activationCode.value = "";
    feedback.open({ type: "success", message: "卡密激活成功" });
  } catch {
    // 错误由 http 层处理
  } finally {
    activationLoading.value = false;
  }
}

onMounted(() => {
  loadDevices();
});
</script>

<style scoped>
.dashboard {
  display: flex;
  flex-direction: column;
  gap: var(--md-spacing-md, 24px);
}

.dash-grid-2col {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--md-spacing-md, 24px);
}

@media (max-width: 900px) {
  .dash-grid-2col {
    grid-template-columns: 1fr;
  }
}

.user-profile {
  display: flex;
  align-items: center;
  gap: var(--md-spacing-md, 24px);
  margin-top: var(--md-spacing-md, 24px);
}

.avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  overflow: hidden;
  background: var(--md-sys-color-primary-container, #eaddff);
  display: flex;
  align-items: center;
  justify-content: center;
}
.avatar img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.avatar-fallback {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--md-sys-color-on-primary-container, #21005d);
}

.user-meta {
  flex: 1;
}
.user-name {
  font-size: 1.25rem;
  font-weight: 700;
}
.user-detail {
  font-size: 0.875rem;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  margin-top: 4px;
}

.device-header {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--md-spacing-sm, 12px);
}

.device-add {
  display: flex;
  gap: var(--md-spacing-sm, 12px);
  align-items: flex-end;
  flex-shrink: 0;
  margin-left: auto;
}

.device-list {
  margin-top: var(--md-spacing-md, 24px);
}
.device-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px 8px;
  border-radius: 8px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant, #cac4d0);
  transition:
    background var(--md-sys-motion-duration-short2, 100ms) var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1)),
    padding-left var(--md-sys-motion-duration-short3, 150ms) var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.device-item:hover {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
  padding-left: 12px;
}
.device-icon-wrapper {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--md-sys-color-secondary-container, #e8def8);
  display: flex;
  align-items: center;
  justify-content: center;
}
.device-info {
  flex: 1;
}
.device-name {
  font-size: 1rem;
  font-weight: 500;
}
.device-sub {
  font-size: 0.8125rem;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}
.projects-header {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--md-spacing-sm, 12px);
}
.activation-row {
  display: flex;
  gap: var(--md-spacing-sm, 12px);
  align-items: flex-end;
  flex-shrink: 0;
  margin-left: auto;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
}
.data-table th,
.data-table td {
  text-align: left;
  padding: 14px 16px;
  border-bottom: 1px solid var(--md-sys-color-outline-variant, #cac4d0);
}
.data-table th {
  font-weight: 500;
  font-size: 0.8125rem;
  letter-spacing: 0.025em;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}
.data-table td {
  font-size: 0.9375rem;
}
.data-table tbody tr {
  transition: background var(--md-sys-motion-duration-short2, 100ms) var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.data-table tbody tr:hover {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
}
</style>
