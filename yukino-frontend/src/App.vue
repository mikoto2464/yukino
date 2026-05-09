<template>
  <AppSnackbar />
  <RouterView />
</template>

<script lang="ts" setup>
import { RouterView } from "vue-router";
import AppSnackbar from "@/components/AppSnackbar.vue";
</script>
