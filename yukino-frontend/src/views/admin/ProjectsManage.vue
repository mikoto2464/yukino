<template>
  <div class="projects-manage">
    <MdCard style="margin-bottom: var(--md-spacing-md, 24px)">
      <div style="display:flex;align-items:center;justify-content:space-between">
        <h1 style="font: var(--md-sys-typescale-headline-small)">项目管理</h1>
        <MdButton variant="filled-tonal" @click="handleAdd">
          新增项目
        </MdButton>
      </div>

      <table class="data-table" style="margin-top: var(--md-spacing-md, 24px)">
        <thead>
          <tr>
            <th>项目名</th>
            <th>状态</th>
            <th>版本</th>
            <th>发布时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="p in pagedItems" :key="p.id">
            <td class="project-name">{{ p.name }}</td>
            <td>
              <MdChip :variant="chipVariantForStatus(p.status)">
                {{ statusLabel(p.status) }}
              </MdChip>
            </td>
            <td>{{ p.version }}</td>
            <td>{{ p.releasedAt }}</td>
            <td>
              <div class="action-cell">
                <MdButton variant="text" @click="handleEdit(p.id)">
                  编辑
                </MdButton>
                <MdButton
                  variant="text"
                  style="color: var(--md-sys-color-error, #b3261e)"
                  :disabled="deletingId === p.id"
                  @click="handleDelete(p.id)"
                >
                  删除
                </MdButton>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- 分页 -->
      <div
        style="display:flex;align-items:center;justify-content:space-between;margin-top:var(--md-spacing-md, 24px);flex-wrap:wrap;gap:var(--md-spacing-sm, 12px)"
      >
        <span style="font: var(--md-sys-typescale-body-small); color: var(--md-sys-color-on-surface-variant)">
          共 {{ projects.length }} 条
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
    </MdCard>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import MdCard from "@/components/md/MdCard.vue";
import MdButton from "@/components/md/MdButton.vue";
import MdChip from "@/components/md/MdChip.vue";
import { useFeedbackStore } from "@/stores/feedback";

type ProjectStatus = "active" | "archived" | "draft";

interface ProjectItem {
  id: string;
  name: string;
  status: ProjectStatus;
  version: string;
  releasedAt: string;
}

const feedback = useFeedbackStore();

const statusMeta: Record<ProjectStatus, { label: string; variant: "assist" | "filter" }> = {
  active: { label: "运行中", variant: "assist" },
  archived: { label: "已归档", variant: "filter" },
  draft: { label: "草稿", variant: "filter" },
};

function chipVariantForStatus(s: ProjectStatus) {
  return statusMeta[s]?.variant ?? "assist";
}
function statusLabel(s: ProjectStatus) {
  return statusMeta[s]?.label ?? s;
}

const projects = ref<ProjectItem[]>([
  { id: "p-1", name: "Yukino Agent", status: "active", version: "v2.3.1", releasedAt: "2026-04-10" },
  { id: "p-2", name: "Yukino Launcher", status: "active", version: "v1.9.8", releasedAt: "2026-03-22" },
  { id: "p-3", name: "Yukino Monitor", status: "draft", version: "v0.9.0-beta", releasedAt: "2026-05-01" },
  { id: "p-4", name: "Yukino Console", status: "archived", version: "v1.2.0", releasedAt: "2025-11-15" },
  { id: "p-5", name: "Yukino Mobile Bridge", status: "active", version: "v3.0.2", releasedAt: "2026-04-28" },
]);

const page = ref(1);
const itemsPerPage = 8;
const deletingId = ref("");

const pageCount = computed(() =>
  Math.max(1, Math.ceil(projects.value.length / itemsPerPage)),
);

const pagedItems = computed(() => {
  const start = (page.value - 1) * itemsPerPage;
  return projects.value.slice(start, start + itemsPerPage);
});

function handleAdd() {
  feedback.open({ type: "info", message: "新增项目功能待接入" });
}

function handleEdit(id: string) {
  feedback.open({ type: "info", message: `编辑项目 ${id} 功能待接入` });
}

function handleDelete(id: string) {
  deletingId.value = id;
  setTimeout(() => {
    projects.value = projects.value.filter((p) => p.id !== id);
    deletingId.value = "";
    if (page.value > pageCount.value) page.value = pageCount.value;
    feedback.open({ type: "success", message: "项目已删除" });
  }, 600);
}
</script>

<style scoped>
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

.project-name {
  font-weight: 500;
}

.action-cell {
  display: flex;
  gap: 4px;
}

.paginator {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--md-spacing-sm, 12px);
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
