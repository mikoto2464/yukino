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
    <slot />
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
    iconSize: 20,
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
  // 增加水平内边距，避免中文文字显得拥挤
  s["--_container-padding-horizontal"] = "32px";
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
