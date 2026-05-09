<template>
  <span
    class="md-icon"
    :style="{
      fontSize: size !== undefined ? `${size}px` : undefined,
      fontVariationSettings: variationSettings,
    }"
    aria-hidden="true"
  >
    {{ icon }}
  </span>
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

const variationSettings = computed(() => {
  const fill = props.filled ? 1 : 0;
  return `'FILL' ${fill}, 'wght' ${props.weight}, 'GRAD' ${props.grade}, 'opsz' ${props.opticalSize}`;
});
</script>

<style scoped>
.md-icon {
  font-family: "Material Symbols Outlined", sans-serif;
  font-weight: normal;
  font-style: normal;
  font-size: 24px;
  line-height: 1;
  letter-spacing: normal;
  text-transform: none;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  white-space: nowrap;
  word-wrap: normal;
  direction: ltr;
  -webkit-font-smoothing: antialiased;
  font-variation-settings: "FILL" 0, "wght" 400, "GRAD" 0, "opsz" 24;
  user-select: none;
  flex-shrink: 0;
}
</style>
