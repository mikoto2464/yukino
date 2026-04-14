<template>
  <v-snackbar
    v-model="feedback.visible"
    :color="snackbarColor"
    :timeout="feedback.timeout"
    location="top left"
    rounded="pill"
    variant="flat"
    class="ma-4 yukino-snackbar"
  >
    <div class="d-flex align-center">
      <v-icon :icon="snackbarIcon" class="me-2" />
      <span class="text-body-2 font-weight-medium">{{ feedback.message }}</span>
    </div>

    <template #actions>
      <v-btn
        icon="mdi-close"
        variant="text"
        density="comfortable"
        @click="feedback.close"
      />
    </template>
  </v-snackbar>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFeedbackStore } from '../stores/feedback'

const feedback = useFeedbackStore()

const snackbarColor = computed(() => {
  if (feedback.type === 'success') {
    return 'success'
  }

  if (feedback.type === 'error') {
    return 'error'
  }

  return 'info'
})

const snackbarIcon = computed(() => {
  if (feedback.type === 'success') {
    return 'mdi-check-circle'
  }

  if (feedback.type === 'error') {
    return 'mdi-alert-circle'
  }

  return 'mdi-information'
})
</script>

<style scoped>
.yukino-snackbar :deep(.v-snackbar__wrapper) {
  box-shadow: 0 8px 22px rgba(0, 0, 0, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.28);
  backdrop-filter: blur(8px);
  opacity: 0.98;
}
</style>
