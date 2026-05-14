<template>
  <button
    ref="hostRef"
    class="md-icon-btn"
    :class="[`md-icon-btn--${variant}`]"
    :aria-label="ariaLabel || icon"
    :disabled="disabled"
    :type="type"
    @click="$emit('click', $event)"
  >
    <span class="md-icon-btn__state-layer"></span>
    <md-icon
      :icon="icon"
      :size="iconSize"
      :filled="selected || toggle"
      class="md-icon-btn__icon"
    />
  </button>
</template>

<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from "vue";
import { useRipple, removeRipple } from "@/composables/useRipple";
import MdIcon from "./MdIcon.vue";

withDefaults(
  defineProps<{
    icon: string;
    variant?: "standard" | "filled" | "filled-tonal" | "outlined";
    selected?: boolean;
    toggle?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    type?: "button" | "submit" | "reset";
    iconSize?: number;
  }>(),
  {
    variant: "standard",
    selected: false,
    toggle: false,
    disabled: false,
    type: "button",
    iconSize: 28,
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

defineExpose({ el: hostRef });
</script>

<style scoped>
.md-icon-btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--md-comp-icon-button-size, 40px);
  height: var(--md-comp-icon-button-size, 40px);
  border: none;
  border-radius: var(--md-sys-shape-corner-full, 9999px);
  background: transparent;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  cursor: pointer;
  overflow: hidden;
  flex-shrink: 0;
}

/* ---- State layer ---- */
.md-icon-btn__state-layer {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background-color: currentColor;
  opacity: 0;
  pointer-events: none;
  transition: opacity
    var(--md-sys-motion-duration-short4, 200ms)
    var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
.md-icon-btn:hover > .md-icon-btn__state-layer {
  opacity: var(--md-sys-state-hover-state-layer-opacity, 0.08);
}
.md-icon-btn:focus-visible > .md-icon-btn__state-layer {
  opacity: var(--md-sys-state-focus-state-layer-opacity, 0.12);
}
.md-icon-btn:active > .md-icon-btn__state-layer {
  opacity: var(--md-sys-state-press-state-layer-opacity, 0.12);
}

/* ---- Disabled ---- */
.md-icon-btn:disabled {
  opacity: 0.38;
  cursor: not-allowed;
  pointer-events: none;
}
.md-icon-btn:disabled > .md-icon-btn__state-layer {
  opacity: 0;
}

/* ---- Icon ---- */
.md-icon-btn__icon {
  position: relative;
  z-index: 1;
}

/* ======== STANDARD (default) ======== */
/* ======== FILLED ======== */
.md-icon-btn--filled {
  background: var(--md-sys-color-primary, #6750a4);
  color: var(--md-sys-color-on-primary, #ffffff);
}
/* ======== FILLED TONAL ======== */
.md-icon-btn--filled-tonal {
  background: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
}
/* ======== OUTLINED ======== */
.md-icon-btn--outlined {
  border: 1px solid var(--md-sys-color-outline, #79747e);
}
</style>
