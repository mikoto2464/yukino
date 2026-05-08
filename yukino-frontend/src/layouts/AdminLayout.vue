<template>
  <!-- 管理端布局：顶部导航栏 + 子导航标签 + 内容区 -->
  <div class="admin-layout">
    <!-- MD3 Top App Bar -->
    <header class="top-bar">
      <div class="top-bar-start">
        <RouterLink to="/" class="brand-link">Yukino</RouterLink>
        <span class="admin-badge">管理</span>
        <nav class="top-bar-nav">
          <RouterLink to="/admin" class="nav-link" exact>卡密生成</RouterLink>
          <RouterLink to="/admin/projects" class="nav-link">
            项目管理
          </RouterLink>
          <RouterLink to="/admin/users" class="nav-link">用户管理</RouterLink>
        </nav>
      </div>
      <div class="top-bar-end">
        <ThemeControls />
        <button
          class="btn btn-text"
          :disabled="logoutLoading"
          @click="handleLogout"
        >
          退出登录
        </button>
      </div>
    </header>

    <!-- 内容区 -->
    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { RouterLink, RouterView, useRouter } from "vue-router";
import ThemeControls from "@/components/ThemeControls.vue";
import { useAuthStore } from "@/stores/auth";
import { useFeedbackStore } from "@/stores/feedback";
import { deleteAuthSession } from "@/api/services";

const router = useRouter();
const auth = useAuthStore();
const feedback = useFeedbackStore();
const logoutLoading = ref(false);

async function handleLogout() {
  logoutLoading.value = true;
  try {
    await deleteAuthSession();
    auth.clearAuth();
    feedback.open({ type: "success", message: "已退出登录" });
    await router.push({ name: "home" });
  } finally {
    logoutLoading.value = false;
  }
}
</script>

<style scoped>
.admin-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.top-bar {
  position: sticky;
  top: 0;
  z-index: 500;
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 64px;
  padding: 0 24px;
  background: var(--md-sys-color-surface-container, #f3edf7);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: background 0.3s;
}

.top-bar-start {
  display: flex;
  align-items: center;
  gap: 16px;
}

.brand-link {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--md-sys-color-on-surface, #1d1b20);
  text-decoration: none;
}

/* "管理"标签 */
.admin-badge {
  padding: 2px 10px;
  border-radius: 100px;
  font-size: 0.7rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  background: var(--md-sys-color-tertiary-container, #ffd8e4);
  color: var(--md-sys-color-on-tertiary-container, #31111d);
}

.top-bar-nav {
  display: flex;
  gap: 4px;
}

.nav-link {
  padding: 6px 14px;
  border-radius: 20px;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  text-decoration: none;
  transition: background 0.15s;
}
.nav-link:hover,
.nav-link.router-link-exact-active,
.nav-link.router-link-active {
  background: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
}

.top-bar-end {
  display: flex;
  align-items: center;
  gap: 8px;
}

.main-content {
  flex: 1;
  padding: 24px;
  max-width: 1400px;
  width: 100%;
  margin: 0 auto;
}
</style>
