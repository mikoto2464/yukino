<template>
  <button
    ref="hostRef"
    class="md-btn"
    :class="[`md-btn--${variant}`]"
    :disabled="disabled"
    :type="type"
    @click="$emit('click', $event)"
  >
    <span class="md-btn__state-layer"></span>
    <md-icon v-if="icon && !loading" :icon="icon" :size="18" class="md-btn__icon" />
    <md-circular-progress
      v-if="loading"
      :size="18"
      :stroke="2.5"
      class="md-btn__spinner"
    />
    <span v-if="$slots.default" class="md-btn__label">
      <slot />
    </span>
  </button>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from "vue";
import { useRipple, removeRipple } from "@/composables/useRipple";
import MdIcon from "./MdIcon.vue";
import MdCircularProgress from "./MdCircularProgress.vue";

withDefaults(
  defineProps<{
    variant?: "filled" | "filled-tonal" | "outlined" | "text" | "elevated";
    icon?: string;
    loading?: boolean;
    disabled?: boolean;
    type?: "button" | "submit" | "reset";
  }>(),
  {
    variant: "filled",
    loading: false,
    disabled: false,
    type: "button",
  },
);

defineEmits<{
  click: [e: MouseEvent];
}>();

const hostRef = ref<HTMLElement | null>(null);

onMounted(() => {
  useRipple(hostRef.value);
});

onUnmounted(() => {
  removeRipple(hostRef.value);
});
</script>

<style scoped>
.md-btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--md-comp-button-gap, 8px);
  height: var(--md-comp-button-height, 40px);
  min-width: var(--md-comp-button-min-width, 48px);
  padding: 0 var(--md-comp-button-padding-horizontal, 24px);
  border: none;
  border-radius: var(--md-sys-shape-corner-full, 9999px);
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: var(--md-sys-typescale-label-large);
  font-weight: 500;
  line-height: var(--md-comp-button-height, 40px);
  cursor: pointer;
  user-select: none;
  overflow: hidden;
  vertical-align: middle;
}

/* ---- State layer ---- */
.md-btn__state-layer {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  transition: background-color
    var(--md-sys-motion-duration-short4, 200ms)
    var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-btn:hover > .md-btn__state-layer {
  background-color: currentColor;
  opacity: var(--md-sys-state-hover-state-layer-opacity, 0.08);
}
.md-btn:focus-visible > .md-btn__state-layer {
  background-color: currentColor;
  opacity: var(--md-sys-state-focus-state-layer-opacity, 0.12);
}
.md-btn:active > .md-btn__state-layer {
  background-color: currentColor;
  opacity: var(--md-sys-state-press-state-layer-opacity, 0.12);
}

/* ---- Disabled ---- */
.md-btn:disabled {
  opacity: 0.38;
  cursor: not-allowed;
  pointer-events: none;
}
.md-btn:disabled > .md-btn__state-layer {
  opacity: 0;
}

/* ---- Label ---- */
.md-btn__label {
  position: relative;
  z-index: 1;
}
.md-btn__icon {
  position: relative;
  z-index: 1;
  flex-shrink: 0;
}
.md-btn__spinner {
  position: relative;
  z-index: 1;
  flex-shrink: 0;
}

/* ======== FILLED ======== */
.md-btn--filled {
  background-color: var(--md-sys-color-primary, #6750a4);
  color: var(--md-sys-color-on-primary, #ffffff);
  box-shadow: var(--md-sys-elevation-level0);
}
.md-btn--filled:hover {
  box-shadow: var(--md-sys-elevation-level1);
}

/* ======== FILLED TONAL ======== */
.md-btn--filled-tonal {
  background-color: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
}

/* ======== OUTLINED ======== */
.md-btn--outlined {
  background-color: transparent;
  color: var(--md-sys-color-primary, #6750a4);
  border: 1px solid var(--md-sys-color-outline, #79747e);
}

/* ======== TEXT ======== */
.md-btn--text {
  background-color: transparent;
  color: var(--md-sys-color-primary, #6750a4);
  min-width: auto;
  padding: 0 12px;
}

/* ======== ELEVATED ======== */
.md-btn--elevated {
  background-color: var(--md-sys-color-surface-container-low, #f7f2fa);
  color: var(--md-sys-color-primary, #6750a4);
  box-shadow: var(--md-sys-elevation-level1);
}
.md-btn--elevated:hover {
  box-shadow: var(--md-sys-elevation-level2);
}
</style>
