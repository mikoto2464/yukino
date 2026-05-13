<template>
  <div class="theme-ctrls">
    <!-- 深色/浅色切换 -->
    <MdIconButton
      ref="darkBtnRef"
      :icon="isDark ? 'light_mode' : 'dark_mode'"
      :aria-label="isDark ? '切换浅色模式' : '切换深色模式'"
      @click="onToggleDark"
    />

    <!-- 随机背景 -->
    <MdIconButton
      ref="shuffleBtnRef"
      icon="shuffle"
      aria-label="随机背景图"
      @click="onShuffleBg"
    />

    <!-- 壁纸选择下拉 -->
    <div class="bg-menu-wrapper">
      <MdIconButton
        icon="palette"
        aria-label="选择背景图"
      />
      <div class="bg-menu" role="menu">
        <div
          v-for="bg in BACKGROUND_OPTIONS"
          :key="bg.key"
          class="bg-menu-item"
          :class="{ active: theme.state.background === bg.key }"
          role="menuitem"
          @click="onSelectBg(bg.key, $event)"
        >
          <img
            class="bg-thumb"
            :src="bg.thumbUrl"
            :alt="bg.label"
            loading="lazy"
          />
          <span class="bg-label">{{ bg.label }}</span>
          <MdIcon v-if="theme.state.background === bg.key" icon="check" :size="20" class="bg-check" />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import MdIconButton from "@/components/md/MdIconButton.vue";
import MdIcon from "@/components/md/MdIcon.vue";
import { useTheme } from "@/composables/useTheme";
import { BACKGROUND_OPTIONS } from "@/config";

const theme = useTheme();
const isDark = computed(() => theme.effectiveDark());

const darkBtnRef = ref<InstanceType<typeof MdIconButton> | null>(null);
const shuffleBtnRef = ref<InstanceType<typeof MdIconButton> | null>(null);

function getEl(
  comp: InstanceType<typeof MdIconButton> | null,
): HTMLElement | undefined {
  return (comp as { el?: HTMLElement } | null)?.el ?? undefined;
}

async function onToggleDark() {
  await theme.toggleDark(getEl(darkBtnRef.value));
}

async function onShuffleBg() {
  await theme.setRandomBackground(getEl(shuffleBtnRef.value));
}

async function onSelectBg(key: string, e: MouseEvent) {
  // 取下拉菜单菜单项的点击坐标作为扩散原点
  const target = e.currentTarget as HTMLElement;
  await theme.setBackground(key, target);
}
</script>

<style scoped>
.theme-ctrls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.bg-menu-wrapper {
  position: relative;
}
.bg-menu-wrapper::before {
  content: "";
  position: absolute;
  inset: 0;
  top: 100%;
  height: 8px;
  pointer-events: auto;
}
.bg-menu-wrapper:hover .bg-menu {
  display: block;
}
.bg-menu {
  display: none;
  position: absolute;
  top: calc(100% + 8px);
  right: 0;
  min-width: 220px;
  max-height: 340px;
  overflow-y: auto;
  padding: 8px 0;
  border-radius: 16px;
  background: var(--md-sys-color-surface-container, #f3edf7);
  box-shadow: var(--md-sys-elevation-level3);
  z-index: 1000;
}
.bg-menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 6px 16px;
  cursor: pointer;
  transition: background 0.2s;
}
.bg-menu-item:hover {
  background: var(--md-sys-color-surface-container-high, #ece6f0);
}
.bg-menu-item.active {
  background: var(--md-sys-color-secondary-container, #e8def8);
}
.bg-thumb {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  object-fit: cover;
}
.bg-label {
  flex: 1;
  font-size: 0.875rem;
  color: var(--md-sys-color-on-surface, #1d1b20);
}
.bg-check {
  color: var(--md-sys-color-primary, #6750a4);
}
</style>
