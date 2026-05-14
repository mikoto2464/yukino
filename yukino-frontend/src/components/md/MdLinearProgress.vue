<template>
  <md-linear-progress
    :indeterminate="indeterminate || undefined"
    :value="indeterminate ? undefined : clampedProgress"
    role="progressbar"
    aria-label="加载中"
  />
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    progress?: number;
    indeterminate?: boolean;
  }>(),
  {
    progress: 0,
    indeterminate: false,
  },
);

const clampedProgress = computed(() => Math.min(100, Math.max(0, props.progress)));
</script>
