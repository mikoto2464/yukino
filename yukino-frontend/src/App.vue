<template>
  <v-app>
    <RouterView />
    <GlobalSnackbar />
  </v-app>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import GlobalSnackbar from './components/GlobalSnackbar.vue'
</script>
