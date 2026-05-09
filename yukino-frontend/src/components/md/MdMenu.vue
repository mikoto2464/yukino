<template>
  <div class="md-menu-wrapper">
    <div ref="anchorRef" @click="toggle">
      <slot name="trigger" />
    </div>
    <Teleport to="body">
      <Transition name="md-menu">
        <div
          v-if="open"
          class="md-menu"
          :style="menuStyle"
          role="menu"
          @click.stop
        >
          <slot />
        </div>
      </Transition>
    </Teleport>
  </div>
  <!-- 点击外部关闭 -->
  <div v-if="open" class="md-menu__backdrop" @click="close" />
</template>

<script lang="ts" setup>
import { ref, computed, watch } from "vue";

const open = ref(false);
const anchorRef = ref<HTMLElement | null>(null);
const menuPos = ref({ top: 0, left: 0 });

function toggle() {
  if (!open.value) {
    updatePosition();
  }
  open.value = !open.value;
}

function close() {
  open.value = false;
}

function updatePosition() {
  const rect = anchorRef.value?.getBoundingClientRect();
  if (rect) {
    menuPos.value = {
      top: rect.bottom + 4,
      left: rect.right,
    };
  }
}

const menuStyle = computed(() => ({
  top: `${menuPos.value.top}px`,
  left: `${menuPos.value.left}px`,
}));

defineExpose({ open, close });
</script>

<style scoped>
.md-menu-wrapper {
  position: relative;
  display: inline-flex;
}

.md-menu {
  position: fixed;
  z-index: 2100;
  min-width: 160px;
  padding: 4px 0;
  border-radius: var(--md-sys-shape-corner-medium, 12px);
  background: var(--md-sys-color-surface-container, #f3edf7);
  box-shadow: var(--md-sys-elevation-level3);
  transform: translateX(-100%);
}

.md-menu__backdrop {
  position: fixed;
  inset: 0;
  z-index: 2099;
}

/* ---- Menu items innerhalb von MdMenu ---- */
:deep(.md-menu-item) {
  display: block;
  width: 100%;
  text-align: left;
  padding: 10px 16px;
  border: none;
  background: transparent;
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: 0.8125rem;
  color: var(--md-sys-color-on-surface, #1d1b20);
  cursor: pointer;
  white-space: nowrap;
}
:deep(.md-menu-item:hover) {
  background: var(--md-sys-color-surface-container-high, #ece6f0);
}

/* ---- Transition ---- */
.md-menu-enter-active {
  animation: md-menu-in 150ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-menu-leave-active {
  animation: md-menu-out 100ms var(--md-sys-motion-easing-accelerate, cubic-bezier(0.3, 0, 0.8, 0.15));
}

@keyframes md-menu-in {
  from {
    opacity: 0;
    transform: translateX(-100%) scale(0.92);
  }
  to {
    opacity: 1;
    transform: translateX(-100%) scale(1);
  }
}
@keyframes md-menu-out {
  to {
    opacity: 0;
    transform: translateX(-100%) scale(0.92);
  }
}
</style>
