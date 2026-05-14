<template>
  <component
    :is="mwcTag"
    :label="placeholder"
    :disabled="disabled"
    :value="modelValue"
    @input="onChange"
  >
    <md-select-option
      v-for="opt in options"
      :key="opt.value"
      :value="opt.value"
      :disabled="opt.disabled"
    >
      <span slot="headline">{{ opt.label }}</span>
    </md-select-option>
  </component>
</template>

<script lang="ts" setup>
import { computed } from "vue";

export interface SelectOption {
  label: string;
  value: string;
  disabled?: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue: string | null;
    options: SelectOption[];
    placeholder?: string;
    variant?: "filled" | "outlined";
    disabled?: boolean;
  }>(),
  {
    variant: "filled",
    disabled: false,
    modelValue: null,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const variantTagMap: Record<string, string> = {
  filled: "md-filled-select",
  outlined: "md-outlined-select",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-filled-select");

function onChange(e: Event) {
  emit("update:modelValue", (e.target as HTMLInputElement).value);
}
</script>
