<template>
  <div class="theme-ctrls">
    <!-- 深色/浅色切换 -->
    <MdIconButton
      :icon="isDark ? 'light_mode' : 'dark_mode'"
      :aria-label="isDark ? '切换浅色模式' : '切换深色模式'"
      @click="theme.toggleDark()"
    />

    <!-- 随机背景 -->
    <MdIconButton
      icon="shuffle"
      aria-label="随机背景图"
      @click="theme.setRandomBackground()"
    />

    <!-- 壁纸选择下拉 -->
    <div class="bg-menu-wrapper">
      <MdIconButton
        icon="palette"
        aria-label="选择背景图"
        @click="toggleMenu"
        @blur="onTriggerBlur"
      />
      <div
        v-show="showMenu"
        class="bg-menu"
        role="menu"
        @mouseenter="hovering = true"
        @mouseleave="hovering = false"
      >
        <div
          v-for="bg in BACKGROUND_OPTIONS"
          :key="bg.key"
          class="bg-menu-item"
          :class="{ active: theme.state.background === bg.key }"
          role="menuitem"
          @click="selectBg(bg.key)"
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
import { computed, ref, watch } from "vue";
import MdIconButton from "@/components/md/MdIconButton.vue";
import MdIcon from "@/components/md/MdIcon.vue";
import { useTheme } from "@/composables/useTheme";
import { BACKGROUND_OPTIONS } from "@/config";

const theme = useTheme();
const isDark = computed(() => theme.effectiveDark());

const showMenu = ref(false);
const hovering = ref(false);

function toggleMenu() {
  showMenu.value = !showMenu.value;
}

function selectBg(key: string) {
  theme.setBackground(key);
  showMenu.value = false;
}

function onTriggerBlur(e: FocusEvent) {
  // 如果焦点移到了菜单内部，不关闭
  const related = e.relatedTarget as HTMLElement | null;
  if (related && related.closest(".bg-menu")) return;
  showMenu.value = false;
}

// 点击外部关闭
function onClickOutside(e: MouseEvent) {
  const target = e.target as HTMLElement;
  if (!target.closest(".bg-menu-wrapper")) {
    showMenu.value = false;
  }
}

watch(showMenu, (val) => {
  if (val) {
    document.addEventListener("click", onClickOutside);
  } else {
    document.removeEventListener("click", onClickOutside);
  }
});
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
.bg-menu-wrapper:hover .bg-menu {
  display: block;
}
.bg-menu {
  display: block;
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
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
