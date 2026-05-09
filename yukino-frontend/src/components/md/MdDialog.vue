<template>
  <Teleport to="body">
    <Transition name="md-dialog">
      <div
        v-if="open"
        class="md-dialog__overlay"
        @click.self="$emit('update:open', false)"
      >
        <div
          class="md-dialog__surface"
          role="dialog"
          aria-modal="true"
          :aria-label="headline || undefined"
        >
          <!-- 标题区 -->
          <div v-if="headline || $slots.header" class="md-dialog__header">
            <h2 v-if="headline" class="md-dialog__title">{{ headline }}</h2>
            <slot name="header" />
          </div>

          <!-- 内容区 -->
          <div class="md-dialog__content">
            <slot />
          </div>

          <!-- 操作按钮区 -->
          <div v-if="$slots.actions" class="md-dialog__actions">
            <slot name="actions" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script lang="ts" setup>
defineProps<{
  open: boolean;
  headline?: string;
}>();

defineEmits<{
  "update:open": [value: boolean];
}>();
</script>

<style scoped>
.md-dialog__overlay {
  position: fixed;
  inset: 0;
  z-index: 2000;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.4);
  padding: 24px;
}

.md-dialog__surface {
  width: 100%;
  max-width: var(--md-comp-dialog-max-width, 560px);
  max-height: 80vh;
  overflow-y: auto;
  padding: var(--md-comp-dialog-padding, 24px);
  border-radius: 28px;
  background: var(--md-sys-color-surface-container-high, #ece6f0);
  box-shadow: var(--md-sys-elevation-level3);
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.md-dialog__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.md-dialog__title {
  font: var(--md-sys-typescale-headline-small);
  color: var(--md-sys-color-on-surface, #1d1b20);
}

.md-dialog__content {
  font: var(--md-sys-typescale-body-medium);
  color: var(--md-sys-color-on-surface-variant, #49454f);
  flex: 1;
}

.md-dialog__actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* ---- Transition ---- */
.md-dialog-enter-active {
  transition: opacity 200ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-dialog-leave-active {
  transition: opacity 150ms var(--md-sys-motion-easing-accelerate, cubic-bezier(0.3, 0, 0.8, 0.15));
}
.md-dialog-enter-from,
.md-dialog-leave-to {
  opacity: 0;
}
.md-dialog-enter-active .md-dialog__surface {
  animation: md-dialog-in 200ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-dialog-leave-active .md-dialog__surface {
  animation: md-dialog-out 150ms var(--md-sys-motion-easing-accelerate, cubic-bezier(0.3, 0, 0.8, 0.15));
}

@keyframes md-dialog-in {
  from {
    transform: scale(0.92);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}
@keyframes md-dialog-out {
  to {
    transform: scale(0.92);
    opacity: 0;
  }
}
</style>
