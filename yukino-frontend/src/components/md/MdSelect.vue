<template>
  <div class="md-select" :class="[`md-select--${variant}`]">
    <select
      ref="selectRef"
      :value="modelValue"
      class="md-select__control"
      :disabled="disabled"
      @change="onChange"
      @focus="focused = true"
      @blur="focused = false"
    >
      <option v-if="placeholder" value="" disabled>{{ placeholder }}</option>
      <option
        v-for="opt in options"
        :key="opt.value"
        :value="opt.value"
        :disabled="opt.disabled"
      >
        {{ opt.label }}
      </option>
    </select>
    <md-icon icon="arrow_drop_down" :size="20" class="md-select__arrow" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import MdIcon from "./MdIcon.vue";

export interface SelectOption {
  label: string;
  value: string;
  disabled?: boolean;
}

withDefaults(
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

const focused = ref(false);
const selectRef = ref<HTMLSelectElement | null>(null);

function onChange(e: Event) {
  const target = e.target as HTMLSelectElement;
  emit("update:modelValue", target.value);
}
</script>

<style scoped>
.md-select {
  position: relative;
  display: inline-flex;
  align-items: center;
}

.md-select__control {
  width: 100%;
  height: var(--md-comp-text-field-height, 56px);
  padding: 0 40px 0 16px;
  border: none;
  border-radius: var(--md-sys-shape-corner-small, 8px);
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: 1rem;
  color: var(--md-sys-color-on-surface, #1d1b20);
  outline: none;
  cursor: pointer;
  appearance: none;
  -webkit-appearance: none;
}

.md-select__arrow {
  position: absolute;
  right: 8px;
  pointer-events: none;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}

/* ======== FILLED ======== */
.md-select--filled .md-select__control {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
  border-bottom: 1px solid var(--md-sys-color-on-surface-variant, #49454f);
  border-radius: var(--md-sys-shape-corner-small, 8px) var(--md-sys-shape-corner-small, 8px) 0 0;
}
.md-select--filled .md-select__control:focus {
  border-bottom: 2px solid var(--md-sys-color-primary, #6750a4);
}

/* ======== OUTLINED ======== */
.md-select--outlined .md-select__control {
  background: transparent;
  border: 1px solid var(--md-sys-color-outline, #79747e);
}
.md-select--outlined .md-select__control:focus {
  border: 2px solid var(--md-sys-color-primary, #6750a4);
}

/* ---- Disabled ---- */
.md-select__control:disabled {
  opacity: 0.38;
  cursor: not-allowed;
}
</style>
