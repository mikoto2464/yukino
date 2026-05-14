<template>
  <div class="md-menu-wrapper">
    <div :id="triggerId" @click="toggle">
      <slot name="trigger" />
    </div>
    <component
      :is="'md-menu'"
      ref="menuRef"
      :anchor="triggerId"
      positioning="popover"
      @closed="open = false"
      @close="open = false"
    >
      <slot />
    </component>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from "vue";

const open = ref(false);
const menuRef = ref<HTMLElement | null>(null);
const triggerId = `md-menu-trigger-${Math.random().toString(36).slice(2, 9)}`;

function toggle() {
  open.value = !open.value;
  if (open.value) {
    (menuRef.value as any)?.show?.();
  } else {
    (menuRef.value as any)?.close?.();
  }
}

defineExpose({ open });
</script>

<style scoped>
.md-menu-wrapper {
  position: relative;
  display: inline-flex;
}
</style>
