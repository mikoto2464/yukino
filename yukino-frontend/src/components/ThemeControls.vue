<template>
  <div class="theme-ctrls">
    <!-- 深色/浅色切换 -->
    <md-icon-button
      class="ctrl-btn"
      :aria-label="isDark ? '切换浅色模式' : '切换深色模式'"
      @click="theme.toggleDark()"
    >
      <md-icon>{{ isDark ? "light_mode" : "dark_mode" }}</md-icon>
    </md-icon-button>

    <!-- 随机背景 -->
    <md-icon-button
      class="ctrl-btn"
      aria-label="随机背景图"
      @click="theme.setRandomBackground()"
    >
      <md-icon>shuffle</md-icon>
    </md-icon-button>

    <!-- 壁纸选择下拉 -->
    <div class="bg-menu-wrapper">
      <md-icon-button
        id="bg-menu-trigger"
        class="ctrl-btn"
        aria-label="选择背景图"
      >
        <md-icon>palette</md-icon>
      </md-icon-button>
      <div class="bg-menu" role="menu">
        <div
          v-for="bg in BACKGROUND_OPTIONS"
          :key="bg.key"
          class="bg-menu-item"
          :class="{ active: theme.state.background === bg.key }"
          role="menuitem"
          @click="theme.setBackground(bg.key)"
        >
          <img
            class="bg-thumb"
            :src="bg.thumbUrl"
            :alt="bg.label"
            loading="lazy"
          />
          <span class="bg-label">{{ bg.label }}</span>
          <md-icon v-if="theme.state.background === bg.key" class="bg-check">
            check
          </md-icon>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import "@material/web/icon/icon.js";
import "@material/web/iconbutton/icon-button.js";
import { computed } from "vue";
import { useTheme } from "@/composables/useTheme";
import { BACKGROUND_OPTIONS } from "@/config";

const theme = useTheme();
const isDark = computed(() => theme.effectiveDark());
</script>

<style scoped>
.theme-ctrls {
  display: flex;
  align-items: center;
  gap: 4px;
}

.ctrl-btn {
  --md-icon-button-icon-size: 24px;
}

/* 壁纸选择弹出菜单 */
.bg-menu-wrapper {
  position: relative;
}
.bg-menu-wrapper:hover .bg-menu,
.bg-menu-wrapper:focus-within .bg-menu {
  display: block;
}
.bg-menu {
  display: none;
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
  box-shadow: var(--md-sys-elevation-level3, 0 4px 8px 3px rgba(0, 0, 0, 0.15));
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
  --md-icon-size: 20px;
  color: var(--md-sys-color-primary);
}
</style>
