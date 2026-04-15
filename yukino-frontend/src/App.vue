<template>
  <v-app>
    <div class="yukino-bg-layer"/>
    <div class="yukino-bg-mask"/>
    <RouterView v-slot="{ Component, route }">
      <Transition mode="out-in" name="route-fade">
        <component :is="Component" :key="route.fullPath"/>
      </Transition>
    </RouterView>
    <GlobalSnackbar/>
  </v-app>
</template>

<script lang="ts" setup>
import {RouterView} from 'vue-router'
import GlobalSnackbar from './components/GlobalSnackbar.vue'
</script>
