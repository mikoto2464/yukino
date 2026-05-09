<template>
  <div class="md-text-field" :class="[`md-text-field--${variant}`]">
    <div class="md-text-field__container">
      <md-icon v-if="icon" :icon="icon" :size="20" class="md-text-field__leading-icon" />
      <div class="md-text-field__input-wrapper">
        <input
          ref="inputRef"
          :type="type"
          :value="modelValue"
          :placeholder="placeholder"
          :disabled="disabled"
          :readonly="readonly"
          class="md-text-field__input"
          @input="onInput"
          @focus="focused = true"
          @blur="focused = false"
        />
        <label
          v-if="label"
          class="md-text-field__label"
          :class="{
            'md-text-field__label--floating': modelValue || focused,
          }"
        >
          {{ label }}
        </label>
      </div>
      <md-icon-button
        v-if="modelValue && clearable"
        icon="close"
        :icon-size="18"
        class="md-text-field__trailing-icon"
        @click="clear"
      />
    </div>
    <div v-if="supportingText" class="md-text-field__supporting" :class="{ 'md-text-field__supporting--error': error }">
      {{ supportingText }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import MdIcon from "./MdIcon.vue";
import MdIconButton from "./MdIconButton.vue";

withDefaults(
  defineProps<{
    modelValue: string;
    label?: string;
    placeholder?: string;
    variant?: "filled" | "outlined";
    type?: "text" | "password" | "email" | "number";
    icon?: string;
    supportingText?: string;
    error?: boolean;
    disabled?: boolean;
    readonly?: boolean;
    clearable?: boolean;
  }>(),
  {
    variant: "filled",
    type: "text",
    error: false,
    disabled: false,
    readonly: false,
    clearable: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const focused = ref(false);
const inputRef = ref<HTMLInputElement | null>(null);

function onInput(e: Event) {
  const target = e.target as HTMLInputElement;
  emit("update:modelValue", target.value);
}

function clear() {
  emit("update:modelValue", "");
  inputRef.value?.focus();
}
</script>

<style scoped>
.md-text-field {
  display: inline-flex;
  flex-direction: column;
  min-width: 200px;
  width: 100%;
}

.md-text-field__container {
  display: flex;
  align-items: center;
  position: relative;
}

.md-text-field__input-wrapper {
  position: relative;
  flex: 1;
}

.md-text-field__input {
  width: 100%;
  height: var(--md-comp-text-field-height, 56px);
  padding: 24px 16px 8px 16px;
  border: none;
  border-radius: var(--md-sys-shape-corner-small, 8px);
  font-family: var(--md-ref-typeface-plain, "Roboto", sans-serif);
  font-size: 1rem;
  line-height: 1.5;
  outline: none;
  color: var(--md-sys-color-on-surface, #1d1b20);
}

.md-text-field__input::placeholder {
  color: var(--md-sys-color-on-surface-variant, #49454f);
  opacity: 0.6;
}

/* ---- Label ---- */
.md-text-field__label {
  position: absolute;
  top: 50%;
  left: 16px;
  transform: translateY(-50%);
  font-size: 1rem;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  pointer-events: none;
  transition: all 150ms var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1));
  transform-origin: left top;
}
.md-text-field__label--floating {
  top: 8px;
  transform: translateY(0) scale(0.75);
}

/* ---- Leading icon ---- */
.md-text-field__leading-icon {
  margin-left: 12px;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  flex-shrink: 0;
}
.md-text-field__leading-icon + .md-text-field__input-wrapper .md-text-field__input {
  padding-left: 12px;
}

/* ---- Trailing icon ---- */
.md-text-field__trailing-icon {
  margin-right: 4px;
  flex-shrink: 0;
}

/* ---- Supporting text ---- */
.md-text-field__supporting {
  font-size: 0.75rem;
  padding: 4px 16px 0;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}
.md-text-field__supporting--error {
  color: var(--md-sys-color-error, #b3261e);
}

/* ======== FILLED ======== */
.md-text-field--filled .md-text-field__input {
  background: var(--md-sys-color-surface-container-highest, #e6e0e9);
  border-bottom: 1px solid var(--md-sys-color-on-surface-variant, #49454f);
  border-radius: var(--md-sys-shape-corner-small, 8px) var(--md-sys-shape-corner-small, 8px) 0 0;
}
.md-text-field--filled .md-text-field__input:focus {
  border-bottom: 2px solid var(--md-sys-color-primary, #6750a4);
}

/* ======== OUTLINED ======== */
.md-text-field--outlined .md-text-field__input {
  background: transparent;
  border: 1px solid var(--md-sys-color-outline, #79747e);
  border-radius: var(--md-sys-shape-corner-small, 8px);
}
.md-text-field--outlined .md-text-field__input:focus {
  border: 2px solid var(--md-sys-color-primary, #6750a4);
}

/* ---- Disabled ---- */
.md-text-field__input:disabled {
  opacity: 0.38;
  cursor: not-allowed;
}
</style>
