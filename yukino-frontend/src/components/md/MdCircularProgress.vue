<template>
  <md-circular-progress
    :indeterminate="indeterminate || undefined"
    :value="indeterminate ? undefined : Math.min(1, Math.max(0, progress))"
    :style="{ '--md-circular-progress-size': `${size}px` }"
    role="progressbar"
    aria-label="加载中"
  />
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    size?: number;
    stroke?: number;
    indeterminate?: boolean;
    progress?: number;
  }>(),
  {
    size: 48,
    stroke: 4,
    indeterminate: true,
    progress: 0,
  },
);
</script>
