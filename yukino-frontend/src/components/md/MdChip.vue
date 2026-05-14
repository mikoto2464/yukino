<template>
  <component :is="mwcTag" :class="{ 'md-chip--clickable': clickable }">
    <md-icon v-if="icon" slot="icon" :icon="icon" :size="20" />
    <slot />
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import MdIcon from "./MdIcon.vue";

const props = withDefaults(
  defineProps<{
    variant?: "assist" | "filter" | "input" | "suggestion";
    icon?: string;
    clickable?: boolean;
    closable?: boolean;
  }>(),
  {
    variant: "assist",
    clickable: false,
    closable: false,
  },
);

defineEmits<{
  close: [];
}>();

const variantTagMap: Record<string, string> = {
  assist: "md-assist-chip",
  filter: "md-filter-chip",
  input: "md-input-chip",
  suggestion: "md-suggestion-chip",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-assist-chip");
</script>

<style scoped>
.md-chip--clickable {
  cursor: pointer;
}
</style>
