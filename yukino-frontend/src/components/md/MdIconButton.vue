<template>
  <component
    :is="mwcTag"
    :disabled="disabled"
    :type="type"
    :aria-label="ariaLabel || icon"
    @click="$emit('click', $event)"
  >
    <md-icon
      :icon="icon"
      :filled="selected || toggle"
      :size="iconSize"
    />
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import MdIcon from "./MdIcon.vue";

const props = withDefaults(
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

const variantTagMap: Record<string, string> = {
  standard: "md-icon-button",
  filled: "md-filled-icon-button",
  "filled-tonal": "md-filled-tonal-icon-button",
  outlined: "md-outlined-icon-button",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-icon-button");
</script>
