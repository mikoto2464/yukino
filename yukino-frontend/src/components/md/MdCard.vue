<template>
  <component :is="mwcTag">
    <slot />
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    variant?: "elevated" | "filled" | "outlined";
  }>(),
  {
    variant: "elevated",
  },
);

const variantTagMap: Record<string, string> = {
  elevated: "md-elevated-card",
  filled: "md-filled-card",
  outlined: "md-outlined-card",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-elevated-card");
</script>
