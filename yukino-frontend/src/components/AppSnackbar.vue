<template>
  <!-- MD3 Snackbar：全局反馈提示，自右上角出现 -->
  <Teleport to="body">
    <Transition name="snackbar">
      <div
        v-if="feedback.visible"
        class="snackbar-surface"
        role="status"
        aria-live="polite"
      >
        <md-icon class="snackbar-icon">{{ iconMap[feedback.type] }}</md-icon>
        <span class="snackbar-text">{{ feedback.message }}</span>
        <md-icon-button class="snackbar-dismiss" @click="feedback.close()">
          <md-icon>close</md-icon>
        </md-icon-button>
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts" setup>
// @material/web 必须导入才能注册 custom element
import "@material/web/icon/icon.js";
import "@material/web/iconbutton/icon-button.js";
import { useFeedbackStore } from "@/stores/feedback";

const feedback = useFeedbackStore();

const iconMap: Record<string, string> = {
  success: "check_circle",
  error: "error",
  info: "info",
};
</script>

<style scoped>
/* MD3 Snackbar 样式，位置固定在右上角，使用 CSS 变量响应主题 */
.snackbar-surface {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 9999;
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 200px;
  max-width: 400px;
  padding: 8px 12px 8px 16px;
  border-radius: 12px; /* MD3 shape extra-small */
  background: var(--md-sys-color-inverse-surface, #303033);
  color: var(--md-sys-color-inverse-on-surface, #f3f0f4);
  box-shadow: var(--md-sys-elevation-level3, 0 4px 8px 3px rgba(0, 0, 0, 0.15));
  font-family: "Roboto", "Noto Sans SC", sans-serif;
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.snackbar-icon {
  flex-shrink: 0;
  --md-icon-size: 20px;
}

.snackbar-text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.snackbar-dismiss {
  flex-shrink: 0;
  --md-icon-button-icon-size: 20px;
}

/* 进出动画 */
.snackbar-enter-active {
  animation: snackbar-in 300ms cubic-bezier(0.05, 0.7, 0.1, 1);
}
.snackbar-leave-active {
  animation: snackbar-out 200ms cubic-bezier(0.3, 0, 0.8, 0.15);
}

@keyframes snackbar-in {
  from {
    opacity: 0;
    transform: translateY(-16px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
@keyframes snackbar-out {
  from {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
  to {
    opacity: 0;
    transform: translateY(-8px) scale(0.98);
  }
}
</style>
