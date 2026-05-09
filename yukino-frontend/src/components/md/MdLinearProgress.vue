<template>
  <div
    class="md-linear-progress"
    :class="{ 'md-linear-progress--indeterminate': indeterminate }"
    role="progressbar"
    aria-label="加载中"
  >
    <div class="md-linear-progress__track">
      <div
        v-if="!indeterminate"
        class="md-linear-progress__bar"
        :style="{ width: `${clampedProgress}%` }"
      />
      <div v-else class="md-linear-progress__bar md-linear-progress__bar--animate" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    progress?: number; // 0–100
    indeterminate?: boolean;
  }>(),
  {
    progress: 0,
    indeterminate: false,
  },
);

const clampedProgress = computed(() => Math.min(100, Math.max(0, props.progress)));
</script>

<style scoped>
.md-linear-progress {
  width: 100%;
}

.md-linear-progress__track {
  position: relative;
  height: var(--md-comp-linear-progress-height, 4px);
  border-radius: 2px;
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
  overflow: hidden;
}

.md-linear-progress__bar {
  height: 100%;
  border-radius: 2px;
  background: var(--md-sys-color-primary, #6750a4);
  transition: width 200ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}

.md-linear-progress__bar--animate {
  width: 30%;
  animation: md-linear-indeterminate 1.8s var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1)) infinite;
  transform-origin: 0% 50%;
}

@keyframes md-linear-indeterminate {
  0%   { transform: translateX(-100%) scaleX(0.5); }
  25%  { transform: translateX(-30%) scaleX(1); }
  50%  { transform: translateX(60%) scaleX(1.2); }
  75%  { transform: translateX(150%) scaleX(0.8); }
  100% { transform: translateX(300%) scaleX(0.3); }
}
</style>
