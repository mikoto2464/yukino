<template>
  <div class="md-card" :class="[`md-card--${variant}`]">
    <div class="md-card__state-layer"></div>
    <slot />
  </div>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    variant?: "elevated" | "filled" | "outlined";
  }>(),
  {
    variant: "elevated",
  },
);
</script>

<style scoped>
.md-card {
  position: relative;
  border-radius: var(--md-sys-shape-corner-medium, 12px);
  padding: var(--md-comp-card-padding, 16px);
  overflow: hidden;
}

/* ---- State layer ---- */
.md-card__state-layer {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  transition: background-color
    var(--md-sys-motion-duration-short4, 200ms)
    var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-card:hover > .md-card__state-layer {
  background-color: var(--md-sys-color-on-surface, #1d1b20);
  opacity: var(--md-sys-state-hover-state-layer-opacity, 0.08);
}

/* ======== ELEVATED ======== */
.md-card--elevated {
  background: var(--md-sys-color-surface-container-low, #f7f2fa);
  box-shadow: var(--md-sys-elevation-level1);
}

/* ======== FILLED ======== */
.md-card--filled {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
}

/* ======== OUTLINED ======== */
.md-card--outlined {
  background: transparent;
  border: 1px solid var(--md-sys-color-outline-variant, #cac4d0);
}
</style>
