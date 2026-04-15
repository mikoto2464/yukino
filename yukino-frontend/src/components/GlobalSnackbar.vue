<template>
  <v-snackbar
      v-model="feedback.visible"
      :color="snackbarColor"
      :timeout="feedback.timeout"
      class="ma-4 yukino-snackbar"
      location="top left"
      rounded="pill"
      variant="flat"
  >
    <div class="d-flex align-center">
      <v-icon :icon="snackbarIcon" class="me-2"/>
      <span class="text-body-2 font-weight-medium">{{ feedback.message }}</span>
    </div>

    <template #actions>
      <v-btn
          density="comfortable"
          icon="mdi-close"
          variant="text"
          @click="feedback.close"
      />
    </template>
  </v-snackbar>
</template>

<script lang="ts" setup>
import {computed} from 'vue'
import {useFeedbackStore} from '../stores/feedback'

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
