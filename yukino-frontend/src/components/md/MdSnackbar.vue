<template>
  <Teleport to="body">
    <Transition name="md-snackbar">
      <div
        v-if="visible"
        class="md-snackbar"
        role="status"
        aria-live="polite"
      >
        <md-icon :icon="iconName" :size="20" class="md-snackbar__icon" />
        <span class="md-snackbar__text">{{ message }}</span>
        <md-icon-button
          icon="close"
          :icon-size="18"
          class="md-snackbar__dismiss"
          @click="close"
        />
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import MdIcon from "./MdIcon.vue";
import MdIconButton from "./MdIconButton.vue";

const props = withDefaults(
  defineProps<{
    visible: boolean;
    message: string;
    type?: "success" | "error" | "info";
  }>(),
  {
    type: "info",
  },
);

const emit = defineEmits<{
  close: [];
}>();

const iconMap: Record<"success" | "error" | "info", string> = {
  success: "check_circle",
  error: "error",
  info: "info",
};

const iconName = computed(() => iconMap[props.type]);

function close() {
  emit("close");
}

defineExpose({ close });
</script>

<style scoped>
.md-snackbar {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 9999;
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 200px;
  max-width: var(--md-comp-snackbar-max-width, 360px);
  padding: 8px 12px 8px 16px;
  border-radius: var(--md-sys-shape-corner-medium, 12px);
  background: var(--md-sys-color-inverse-surface, #303033);
  color: var(--md-sys-color-inverse-on-surface, #f3f0f4);
  box-shadow: var(--md-sys-elevation-level3);
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: 0.875rem;
  line-height: 1.25rem;
}

.md-snackbar__icon {
  flex-shrink: 0;
}

.md-snackbar__text {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.md-snackbar__dismiss {
  flex-shrink: 0;
  color: var(--md-sys-color-inverse-on-surface, #f3f0f4);
}

/* ---- Transition ---- */
.md-snackbar-enter-active {
  animation: md-snackbar-in 300ms var(--md-sys-motion-easing-emphasized, cubic-bezier(0.05, 0.7, 0.1, 1));
}
.md-snackbar-leave-active {
  animation: md-snackbar-out 200ms var(--md-sys-motion-easing-accelerate, cubic-bezier(0.3, 0, 0.8, 0.15));
}

@keyframes md-snackbar-in {
  from {
    opacity: 0;
    transform: translateY(-16px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
@keyframes md-snackbar-out {
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
