<template>
  <component :is="mwcTag">
    <div class="md-card__content">
      <slot />
    </div>
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    variant?: "elevated" | "filled" | "outlined";
  }>(),
  {
    variant: "filled",
  },
);

const variantTagMap: Record<string, string> = {
  elevated: "md-elevated-card",
  filled: "md-filled-card",
  outlined: "md-outlined-card",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-filled-card");
</script>

<style scoped>
.md-card__content {
  padding: var(--md-comp-card-padding, 24px);
  transition: background var(--md-sys-motion-duration-short3, 150ms) var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
}
</style>
