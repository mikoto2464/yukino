<template>
  <component
    :is="'md-dialog'"
    ref="dialogRef"
    @close="onClose"
  >
    <span v-if="headline" slot="headline">{{ headline }}</span>
    <slot v-if="$slots.header" slot="headline" name="header" />
    <div slot="content">
      <slot />
    </div>
    <div v-if="$slots.actions" slot="actions">
      <slot name="actions" />
    </div>
  </component>
</template>

<script lang="ts" setup>
import { ref, watch, onMounted, nextTick } from "vue";

const props = defineProps<{
  open: boolean;
  headline?: string;
}>();

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

const dialogRef = ref<HTMLElement | null>(null);

onMounted(async () => {
  if (props.open) {
    await nextTick();
    (dialogRef.value as any)?.show?.();
  }
});

watch(
  () => props.open,
  async (val) => {
    if (!dialogRef.value) return;
    await nextTick();
    if (val) {
      (dialogRef.value as any)?.show?.();
    } else {
      (dialogRef.value as any)?.close?.();
    }
  },
);

function onClose() {
  emit("update:open", false);
}
</script>
