<template>
  <section class="public-hero">
    <div class="hero-content">
      <h1 class="hero-title">Yukino</h1>
      <p class="hero-desc">Mikoto 的个人软件分发中心。</p>
      <div class="hero-actions">
        <RouterLink to="/dashboard" v-if="auth.isAuthenticated" class="hero-btn-link">
          <MdButton variant="filled">前往控制台</MdButton>
        </RouterLink>
        <RouterLink to="/login" v-else class="hero-btn-link">
          <MdButton variant="filled">登录</MdButton>
        </RouterLink>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import { RouterLink } from "vue-router";
import MdButton from "@/components/md/MdButton.vue";
import { useAuthStore } from "@/stores/auth";

const auth = useAuthStore();
</script>

<style scoped>
.public-hero {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - var(--md-comp-top-app-bar-height, 64px) - var(--md-spacing-lg, 32px));
  padding: 24px;
}

.hero-content {
  text-align: center;
  max-width: 480px;
}

.hero-title {
  font: var(--md-sys-typescale-display-large);
  color: var(--md-sys-color-primary, #6750a4);
  animation: hero-title-in 600ms var(--md-sys-motion-easing-emphasized, cubic-bezier(0.05, 0.7, 0.1, 1)) both;
}

.hero-desc {
  margin-top: 12px;
  font: var(--md-sys-typescale-body-large);
  color: var(--md-sys-color-on-surface-variant, #49454f);
  animation: hero-fade-up 600ms 150ms var(--md-sys-motion-easing-emphasized, cubic-bezier(0.05, 0.7, 0.1, 1)) both;
}

.hero-actions {
  margin-top: 32px;
  display: flex;
  gap: 12px;
  justify-content: center;
  animation: hero-fade-up 600ms 300ms var(--md-sys-motion-easing-emphasized, cubic-bezier(0.05, 0.7, 0.1, 1)) both;
}

@keyframes hero-title-in {
  from {
    opacity: 0;
    transform: translateY(-24px) scale(0.94);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}
@keyframes hero-fade-up {
  from {
    opacity: 0;
    transform: translateY(12px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.hero-btn-link {
  text-decoration: none;
}
</style>
