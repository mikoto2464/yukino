<template>
  <component
    :is="mwcTag"
    :disabled="disabled || loading"
    :type="type"
    :trailing-icon="iconPosition === 'end' || undefined"
    :style="hostStyle"
    @click="$emit('click', $event)"
  >
    <md-circular-progress
      v-if="loading"
      indeterminate
      slot="icon"
      :style="{ width: '18px', height: '18px' }"
    />
    <md-icon v-else-if="icon" slot="icon" :icon="icon" :size="iconSize" />
    <span class="btn-label"><slot /></span>
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    variant?: "filled" | "filled-tonal" | "outlined" | "text" | "elevated";
    icon?: string;
    iconPosition?: "start" | "end";
    iconSize?: number;
    loading?: boolean;
    disabled?: boolean;
    type?: "button" | "submit" | "reset";
    rippleColor?: string;
    cornerRadius?: number | string;
  }>(),
  {
    variant: "filled",
    iconPosition: "start",
    iconSize: 24,
    loading: false,
    disabled: false,
    type: "button",
  },
);

defineEmits<{
  click: [e: MouseEvent];
}>();

const variantTagMap: Record<string, string> = {
  filled: "md-filled-button",
  "filled-tonal": "md-filled-tonal-button",
  outlined: "md-outlined-button",
  text: "md-text-button",
  elevated: "md-elevated-button",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-filled-button");

const hostStyle = computed(() => {
  const s: Record<string, string> = {};
  if (props.cornerRadius !== undefined) {
    const r = typeof props.cornerRadius === "number" ? `${props.cornerRadius}px` : props.cornerRadius;
    s["--md-filled-button-container-shape"] = r;
    s["--md-filled-tonal-button-container-shape"] = r;
    s["--md-outlined-button-container-shape"] = r;
    s["--md-text-button-container-shape"] = r;
    s["--md-elevated-button-container-shape"] = r;
  }
  return s;
});
</script>

<style>
/* MD3 中号按钮: 高度 56dp, 图标 24dp */
md-filled-button,
md-filled-tonal-button,
md-outlined-button,
md-elevated-button {
  --md-filled-button-container-height: 56px;
  --md-filled-tonal-button-container-height: 56px;
  --md-outlined-button-container-height: 56px;
  --md-elevated-button-container-height: 56px;
  --_container-height: 56px;
  --md-filled-button-icon-size: 24px;
  --md-filled-tonal-button-icon-size: 24px;
  --md-outlined-button-icon-size: 24px;
  --_container-shape: 8px;
  --md-filled-button-container-shape: 8px;
  --md-filled-tonal-button-container-shape: 8px;
  --md-outlined-button-container-shape: 8px;
  --md-elevated-button-container-shape: 8px;
}
</style>

<style scoped>
.btn-label {
  display: inline-block;
  padding-inline: 16px;
}
</style>
