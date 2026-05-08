<template>
  <!-- 全局 Snackbar 反馈 -->
  <AppSnackbar />
  <!-- 路由内容由布局组件渲染 -->
  <RouterView />
</template>

<script lang="ts" setup>
import { RouterView } from "vue-router";
import AppSnackbar from "@/components/AppSnackbar.vue";
</script>
