<template>
  <div class="cdkeys">
    <!-- 卡密生成区 -->
    <MdCard style="margin-bottom: var(--md-spacing-md, 20px)">
      <div style="display:flex;align-items:center;justify-content:space-between">
        <h1 style="font: var(--md-sys-typescale-headline-small)">卡密生成</h1>
        <MdButton variant="filled-tonal" @click="addConfigRow">
          新增项目配置
        </MdButton>
      </div>

      <div v-for="row in formRows" :key="row.id" class="gen-row">
        <MdSelect
          v-model="row.projectId"
          :options="selectOptions"
          placeholder="选择项目"
          style="max-width: 320px"
        />
        <MdSelect
          v-model="row.period"
          :options="periodSelectOptions"
          style="max-width: 160px"
        />
        <MdButton
          variant="text"
          style="color: var(--md-sys-color-error, #b3261e)"
          :disabled="formRows.length === 1"
          @click="removeConfigRow(row.id)"
        >
          删除
        </MdButton>
      </div>

      <MdButton
        variant="filled"
        :disabled="generateLoading"
        :loading="generateLoading"
        style="margin-top: var(--md-spacing-md, 20px)"
        @click="generateKeys"
      >
        生成卡密
      </MdButton>
    </MdCard>

    <!-- 卡密管理列表 -->
    <MdCard>
      <div
        style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:16px"
      >
        <h2 style="font: var(--md-sys-typescale-title-medium)">卡密管理</h2>
        <MdTextField
          v-model="search"
          placeholder="搜索卡密 / 项目 / 操作人"
          style="max-width: 320px"
        />
      </div>

      <!-- 骨架屏 -->
      <div v-if="tableLoading" style="margin-top: 16px">
        <div
          v-for="i in 6"
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
              <th>卡密</th>
              <th>项目周期</th>
              <th>状态</th>
              <th>创建人</th>
              <th>创建时间</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in pagedItems" :key="item.id">
              <td style="font-family: monospace; font-size: 0.8125rem">
                {{ item.key }}
              </td>
              <td>
                <MdChip
                  v-for="b in item.bindings"
                  :key="`${item.id}-${b.projectId}`"
                  variant="assist"
                  style="margin-right: 4px"
                >
                  {{ b.projectName }} · {{ b.period }}
                </MdChip>
              </td>
              <td>
                <MdChip :variant="chipVariantForStatus(item.status)">
                  {{ statusLabel(item.status) }}
                </MdChip>
              </td>
              <td>{{ item.operator }}</td>
              <td>{{ item.createdAt }}</td>
            </tr>
          </tbody>
        </table>

        <!-- 分页 -->
        <div
          style="display:flex;align-items:center;justify-content:space-between;margin-top:16px;flex-wrap:wrap;gap:8px"
        >
          <span style="font: var(--md-sys-typescale-body-small); color: var(--md-sys-color-on-surface-variant)">
            共 {{ filteredItems.length }} 条
          </span>
          <div class="paginator">
            <button
              v-for="n in pageCount"
              :key="n"
              :class="{ active: page === n }"
              @click="page = n"
            >
              {{ n }}
            </button>
          </div>
        </div>
      </template>
    </MdCard>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from "vue";
import MdCard from "@/components/md/MdCard.vue";
import MdButton from "@/components/md/MdButton.vue";
import MdTextField from "@/components/md/MdTextField.vue";
import MdSelect from "@/components/md/MdSelect.vue";
import MdChip from "@/components/md/MdChip.vue";
import type { SelectOption } from "@/components/md/MdSelect.vue";
import { useFeedbackStore } from "@/stores/feedback";

// ---------- 选项与数据 ----------
interface ProjectOption {
  id: string;
  name: string;
}

type PeriodOption = "7天" | "30天" | "90天" | "180天" | "365天";
type KeyStatus = "active" | "used" | "expired";

interface ConfigRow {
  id: string;
  projectId: string | null;
  period: PeriodOption;
}

interface CDKeyBinding {
  projectId: string;
  projectName: string;
  period: PeriodOption;
}

interface CDKeyItem {
  id: string;
  key: string;
  bindings: CDKeyBinding[];
  status: KeyStatus;
  operator: string;
  createdAt: string;
}

const feedback = useFeedbackStore();

const projectOptions: ProjectOption[] = [
  { id: "p-agent", name: "Yukino Agent" },
  { id: "p-launcher", name: "Yukino Launcher" },
  { id: "p-monitor", name: "Yukino Monitor" },
  { id: "p-console", name: "Yukino Console" },
  { id: "p-mobile", name: "Yukino Mobile Bridge" },
];

const projectNameMap = projectOptions.reduce<Record<string, string>>(
  (acc, p) => ({ ...acc, [p.id]: p.name }),
  {},
);

const periodOptions: PeriodOption[] = ["7天", "30天", "90天", "180天", "365天"];

const selectOptions: SelectOption[] = [
  { label: "选择项目", value: "", disabled: true },
  ...projectOptions.map((p) => ({ label: p.name, value: p.id })),
];

const periodSelectOptions: SelectOption[] = periodOptions.map((p) => ({
  label: p,
  value: p,
}));

const statusMeta: Record<KeyStatus, { label: string; variant: "assist" | "filter" }> = {
  active: { label: "可用", variant: "assist" },
  used: { label: "已使用", variant: "filter" },
  expired: { label: "已过期", variant: "filter" },
};

function chipVariantForStatus(s: KeyStatus) {
  return statusMeta[s]?.variant ?? "assist";
}
function statusLabel(s: KeyStatus) {
  return statusMeta[s]?.label ?? s;
}

// ---------- 表单 ----------
const formRows = ref<ConfigRow[]>([
  {
    id: crypto.randomUUID(),
    projectId: projectOptions[0]?.id ?? null,
    period: "30天",
  },
]);

const generateLoading = ref(false);

function addConfigRow() {
  formRows.value.push({
    id: crypto.randomUUID(),
    projectId: null,
    period: "30天",
  });
}

function removeConfigRow(id: string) {
  if (formRows.value.length === 1) return;
  formRows.value = formRows.value.filter((r) => r.id !== id);
}

function randomToken(): string {
  const chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
  const seg = () =>
    Array.from(
      { length: 4 },
      () => chars[Math.floor(Math.random() * chars.length)],
    ).join("");
  return `YKN-${seg()}-${seg()}-${seg()}`;
}

function generateKeys() {
  const valid = formRows.value.filter((r) => r.projectId);
  if (valid.length === 0) {
    feedback.open({ type: "error", message: "请至少配置一个有效项目" });
    return;
  }

  generateLoading.value = true;
  setTimeout(() => {
    const now = new Date().toLocaleString("zh-CN", { hour12: false });
    const newRows: CDKeyItem[] = valid.map((row) => {
      const pid = row.projectId!;
      return {
        id: crypto.randomUUID(),
        key: randomToken(),
        bindings: [
          {
            projectId: pid,
            projectName: projectNameMap[pid] ?? pid,
            period: row.period,
          },
        ],
        status: "active",
        operator: "admin_01",
        createdAt: now,
      };
    });
    keyItems.value = [...newRows, ...keyItems.value];
    generateLoading.value = false;
    page.value = 1;
    feedback.open({
      type: "success",
      message: `已生成 ${newRows.length} 条卡密`,
    });
  }, 800);
}

// ---------- 列表 ----------
const search = ref("");
const tableLoading = ref(true);
const page = ref(1);
const itemsPerPage = 8;

const keyItems = ref<CDKeyItem[]>([
  {
    id: "k-1",
    key: "YKN-7A3B-91KC-5PQM",
    bindings: [
      { projectId: "p-agent", projectName: "Yukino Agent", period: "30天" },
    ],
    status: "active",
    operator: "admin_01",
    createdAt: "2026-04-15 09:12",
  },
  {
    id: "k-2",
    key: "YKN-3MX9-8QQE-1NTR",
    bindings: [
      {
        projectId: "p-launcher",
        projectName: "Yukino Launcher",
        period: "90天",
      },
    ],
    status: "used",
    operator: "admin_01",
    createdAt: "2026-04-15 09:35",
  },
  {
    id: "k-3",
    key: "YKN-Z11P-6HRT-2BVA",
    bindings: [
      {
        projectId: "p-monitor",
        projectName: "Yukino Monitor",
        period: "30天",
      },
      {
        projectId: "p-console",
        projectName: "Yukino Console",
        period: "30天",
      },
    ],
    status: "active",
    operator: "ops_02",
    createdAt: "2026-04-15 10:20",
  },
]);

setTimeout(() => {
  tableLoading.value = false;
}, 500);

const filteredItems = computed(() => {
  const kw = search.value.trim().toLowerCase();
  if (!kw) return keyItems.value;
  return keyItems.value.filter((item) => {
    const bindingText = item.bindings
      .map((b) => `${b.projectName} ${b.period}`)
      .join(" ");
    return [item.key, item.operator, item.createdAt, bindingText]
      .join(" ")
      .toLowerCase()
      .includes(kw);
  });
});

const pageCount = computed(() =>
  Math.max(1, Math.ceil(filteredItems.value.length / itemsPerPage)),
);

const pagedItems = computed(() => {
  const start = (page.value - 1) * itemsPerPage;
  return filteredItems.value.slice(start, start + itemsPerPage);
});

watch(filteredItems, () => {
  if (page.value > pageCount.value) page.value = 1;
});
</script>

<style scoped>
.gen-row {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 16px;
  flex-wrap: wrap;
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

.paginator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}
.paginator button {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  cursor: pointer;
  font-weight: 500;
  transition: background 0.15s;
}
.paginator button:hover {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
}
.paginator button.active {
  background: var(--md-sys-color-primary-container, #eaddff);
  color: var(--md-sys-color-on-primary-container, #21005d);
}
</style>
