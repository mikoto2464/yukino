<template>
  <svg
    class="md-circular-progress"
    :class="{ 'md-circular-progress--indeterminate': indeterminate }"
    :width="size"
    :height="size"
    :viewBox="`0 0 ${size} ${size}`"
    role="progressbar"
    aria-label="加载中"
  >
    <!-- 轨道圆环 -->
    <circle
      class="md-circular-progress__track"
      :cx="center"
      :cy="center"
      :r="radius"
      :stroke-width="stroke"
      fill="none"
    />
    <!-- 进度弧 -->
    <circle
      class="md-circular-progress__arc"
      :class="{ 'md-circular-progress__arc--animate': indeterminate }"
      :cx="center"
      :cy="center"
      :r="radius"
      :stroke-width="stroke"
      :stroke-dasharray="circumference"
      :stroke-dashoffset="indeterminate ? circumference * 0.75 : offset"
      fill="none"
      stroke-linecap="round"
    />
  </svg>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    size?: number;
    stroke?: number;
    indeterminate?: boolean;
    /** 进度 0–1，仅在 !indeterminate 时有效 */
    progress?: number;
  }>(),
  {
    size: 48,
    stroke: 4,
    indeterminate: true,
    progress: 0,
  },
);

const center = computed(() => props.size / 2);
const radius = computed(() => (props.size - props.stroke) / 2);
const circumference = computed(() => 2 * Math.PI * radius.value);
const offset = computed(() => circumference.value * (1 - Math.min(1, Math.max(0, props.progress))));
</script>

<style scoped>
.md-circular-progress {
  display: block;
  flex-shrink: 0;
}

.md-circular-progress__track {
  stroke: var(--md-sys-color-surface-container-highest, #e6e0e9);
}

.md-circular-progress__arc {
  stroke: var(--md-sys-color-primary, #6750a4);
  transform-origin: 50% 50%;
}

.md-circular-progress__arc--animate {
  animation:
    md-circular-spin 1.4s linear infinite,
    md-circular-arc 1.4s var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1)) infinite;
}

@keyframes md-circular-spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes md-circular-arc {
  0% {
    stroke-dasharray: 1, 200;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 90, 200;
    stroke-dashoffset: -35;
  }
  100% {
    stroke-dasharray: 90, 200;
    stroke-dashoffset: -125;
  }
}
</style>
