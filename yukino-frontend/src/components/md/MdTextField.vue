<template>
  <div class="md-text-field">
    <component
      :is="mwcTag"
      :label="label"
      :placeholder="placeholder"
      :disabled="disabled"
      :readonly="readonly || undefined"
      :error="error || undefined"
      :type="type"
      :value="modelValue"
      :maxlength="maxLength || undefined"
      :prefix-text="prefixText || undefined"
      :suffix-text="suffixText || undefined"
      @input="onInput"
    >
      <md-icon v-if="icon" slot="leading-icon" :icon="icon" :size="20" />
      <md-icon-button
        v-if="showClear"
        slot="trailing-icon"
        icon="close"
        :icon-size="20"
        tabindex="-1"
        @click.stop="clear"
      />
      <slot v-if="$slots.trailing" slot="trailing-icon" name="trailing" />
    </component>
    <div v-if="supportingText || counterText" class="md-text-field__supporting" :class="{ 'md-text-field__supporting--error': error }">
      <span>{{ supportingText }}</span>
      <span v-if="counterText" class="md-text-field__counter">{{ counterText }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from "vue";
import MdIcon from "./MdIcon.vue";
import MdIconButton from "./MdIconButton.vue";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    label?: string;
    placeholder?: string;
    variant?: "filled" | "outlined";
    type?: "text" | "password" | "email" | "number" | "search";
    icon?: string;
    prefixText?: string;
    suffixText?: string;
    supportingText?: string;
    error?: boolean;
    disabled?: boolean;
    readonly?: boolean;
    clearable?: boolean;
    maxLength?: number;
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

const variantTagMap: Record<string, string> = {
  filled: "md-filled-text-field",
  outlined: "md-outlined-text-field",
};

const mwcTag = computed(() => variantTagMap[props.variant] ?? "md-filled-text-field");

const showClear = computed(
  () => props.clearable && !!props.modelValue && !props.disabled && !props.readonly,
);

const counterText = computed(() => {
  if (props.maxLength === undefined) return "";
  const len = props.modelValue?.length ?? 0;
  return `${len} / ${props.maxLength}`;
});

function onInput(e: Event) {
  emit("update:modelValue", (e.target as HTMLInputElement).value);
}

function clear() {
  emit("update:modelValue", "");
}
</script>

<style scoped>
.md-text-field {
  width: 100%;
}

.md-text-field__supporting {
  display: flex;
  align-items: flex-start;
  min-height: 20px;
  padding: 4px 16px 0;
  font-size: 0.75rem;
  line-height: 1.5;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}

.md-text-field__supporting--error {
  color: var(--md-sys-color-error, #b3261e);
}

.md-text-field__counter {
  margin-left: auto;
  flex-shrink: 0;
  padding-left: 16px;
  font-variant-numeric: tabular-nums;
}
</style>
