<template>
  <!-- 首页：Yukino 产品介绍，引导登录或进入控制台 -->
  <section class="public-hero">
    <div class="hero-content">
      <h1 class="hero-title">Yukino</h1>
      <p class="hero-desc">Mikoto 的个人软件分发中心。</p>
      <div class="hero-actions">
        <RouterLink
          v-if="auth.isAuthenticated"
          to="/dashboard"
          class="btn btn-primary"
        >
          前往控制台
        </RouterLink>
        <RouterLink v-else to="/login" class="btn btn-primary">
          登录
        </RouterLink>
      </div>
    </div>
  </section>
</template>

<script lang="ts" setup>
import { RouterLink } from "vue-router";
import { useAuthStore } from "@/stores/auth";

const auth = useAuthStore();
</script>

<style scoped>
.public-hero {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 112px);
  padding: 24px;
}

.hero-content {
  text-align: center;
  max-width: 480px;
}

.hero-title {
  font-size: 3rem;
  font-weight: 800;
  line-height: 1.15;
  color: var(--md-sys-color-primary, #6750a4);
  letter-spacing: -0.02em;
}

.hero-desc {
  margin-top: 12px;
  font-size: 1.125rem;
  color: var(--md-sys-color-on-surface-variant, #49454f);
}

.hero-actions {
  margin-top: 32px;
  display: flex;
  gap: 12px;
  justify-content: center;
}
</style>
