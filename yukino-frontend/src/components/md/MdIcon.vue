<template>
  <component
    :is="'md-icon'"
    :style="iconStyle"
    aria-hidden="true"
  >{{ icon }}</component>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    icon: string;
    size?: number;
    filled?: boolean;
    weight?: number;
    grade?: number;
    opticalSize?: number;
  }>(),
  {
    filled: false,
    weight: 400,
    grade: 0,
    opticalSize: 24,
  },
);

const iconStyle = computed(() => {
  const fill = props.filled ? 1 : 0;
  const s: Record<string, string> = {
    fontVariationSettings: `'FILL' ${fill}, 'wght' ${props.weight}, 'GRAD' ${props.grade}, 'opsz' ${props.opticalSize}`,
  };
  if (props.size !== undefined) {
    s.fontSize = `${props.size}px`;
  }
  return s;
});
</script>
