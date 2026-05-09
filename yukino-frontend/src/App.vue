<template>
  <MdSnackbar
    :visible="feedback.visible"
    :message="feedback.message"
    :type="feedback.type"
    @close="feedback.close()"
  />
  <RouterView />
</template>

<script lang="ts" setup>
import { RouterView } from "vue-router";
import { useFeedbackStore } from "@/stores/feedback";
import MdSnackbar from "@/components/md/MdSnackbar.vue";

const feedback = useFeedbackStore();
</script>
