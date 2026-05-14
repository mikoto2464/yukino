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

<style scoped>
.md-card__content {
  padding: var(--md-comp-card-padding, 24px);
}
</style>
