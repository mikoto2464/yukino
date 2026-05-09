<template>
  <span class="md-chip" :class="[`md-chip--${variant}`, { 'md-chip--clickable': clickable }]">
    <md-icon v-if="icon" :icon="icon" :size="18" class="md-chip__icon" />
    <span class="md-chip__label"><slot /></span>
    <md-icon-button
      v-if="closable"
      icon="close"
      :icon-size="16"
      class="md-chip__close"
      @click.stop="$emit('close')"
    />
  </span>
</template>

<script lang="ts" setup>
import MdIcon from "./MdIcon.vue";
import MdIconButton from "./MdIconButton.vue";

withDefaults(
  defineProps<{
    variant?: "assist" | "filter" | "input" | "suggestion";
    icon?: string;
    clickable?: boolean;
    closable?: boolean;
  }>(),
  {
    variant: "assist",
    clickable: false,
    closable: false,
  },
);

defineEmits<{
  close: [];
}>();
</script>

<style scoped>
.md-chip {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  height: var(--md-comp-chip-height, 32px);
  padding: 0 var(--md-comp-chip-padding-horizontal, 12px);
  border-radius: var(--md-sys-shape-corner-small, 8px);
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: var(--md-sys-typescale-label-medium);
  font-weight: 500;
  user-select: none;
  white-space: nowrap;
}

.md-chip__icon {
  margin-left: -4px;
  flex-shrink: 0;
}

.md-chip__label {
  line-height: var(--md-comp-chip-height, 32px);
}

/* ======== ASSIST ======== */
.md-chip--assist {
  background: var(--md-sys-color-surface-container-low, #f7f2fa);
  color: var(--md-sys-color-on-surface, #1d1b20);
}

/* ======== FILTER ======== */
.md-chip--filter {
  background: var(--md-sys-color-surface-container-low, #f7f2fa);
  color: var(--md-sys-color-on-surface-variant, #49454f);
}

/* ======== INPUT / SUGGESTION ======== */
.md-chip--input,
.md-chip--suggestion {
  background: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
}

/* ======== CLICKABLE ======== */
.md-chip--clickable {
  cursor: pointer;
}
.md-chip--clickable:hover {
  filter: brightness(0.95);
}
</style>
