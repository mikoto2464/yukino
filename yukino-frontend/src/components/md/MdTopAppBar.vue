<template>
  <header class="md-top-app-bar" :class="{ 'md-top-app-bar--scrolled': scrolled }">
    <div class="md-top-app-bar__start">
      <slot name="start" />
    </div>
    <div class="md-top-app-bar__end">
      <slot name="end" />
    </div>
  </header>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    scrolled?: boolean;
  }>(),
  { scrolled: false },
);
</script>

<style scoped>
.md-top-app-bar {
  position: sticky;
  top: 0;
  z-index: 500;
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: var(--md-comp-top-app-bar-height, 64px);
  padding: 0 var(--md-comp-top-app-bar-padding, 16px);
  background: var(--md-sys-color-surface, #fef7ff);
  transition:
    background 300ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1)),
    box-shadow 300ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}

.md-top-app-bar--scrolled {
  background: var(--md-sys-color-surface-container, #f3edf7);
  box-shadow: var(--md-sys-elevation-level2);
}

.md-top-app-bar__start,
.md-top-app-bar__end {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>
