<template>
  <div class="admin-layout">
    <MdTopAppBar>
      <template #start>
        <RouterLink to="/" class="brand-link">Yukino</RouterLink>
        <span class="admin-badge">管理</span>
        <nav class="top-bar-nav">
          <RouterLink to="/admin" class="nav-link">卡密生成</RouterLink>
          <RouterLink to="/admin/projects" class="nav-link">
            项目管理
          </RouterLink>
          <RouterLink to="/admin/users" class="nav-link">用户管理</RouterLink>
        </nav>
      </template>
      <template #end>
        <ThemeControls />
        <MdButton
          variant="text"
          :disabled="logoutLoading"
          @click="handleLogout"
        >
          退出登录
        </MdButton>
      </template>
    </MdTopAppBar>

    <main class="main-content">
      <RouterView v-slot="{ Component }">
        <Transition name="page-fade" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </main>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { RouterLink, RouterView, useRouter } from "vue-router";
import MdTopAppBar from "@/components/md/MdTopAppBar.vue";
import MdButton from "@/components/md/MdButton.vue";
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

.brand-link {
  font-size: 1.375rem;
  font-weight: 700;
  color: var(--md-sys-color-on-surface, #1d1b20);
  text-decoration: none;
}

.admin-badge {
  padding: 3px 12px;
  border-radius: 100px;
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  background: var(--md-sys-color-tertiary-container, #ffd8e4);
  color: var(--md-sys-color-on-tertiary-container, #31111d);
}

.top-bar-nav {
  display: flex;
  gap: 8px;
  margin-left: 16px;
}

.nav-link {
  padding: 8px 16px;
  border-radius: 20px;
  font-size: 0.9375rem;
  font-weight: 500;
  color: var(--md-sys-color-on-surface-variant, #49454f);
  text-decoration: none;
  transition: background 0.15s;
}
.nav-link:hover,
.nav-link.router-link-exact-active {
  background: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
  transform: scale(1.04);
}

.main-content {
  flex: 1;
  padding: var(--md-spacing-lg, 32px);
  max-width: 1440px;
  width: 100%;
  margin: 0 auto;
}

.page-fade-enter-active,
.page-fade-leave-active {
  transition:
    opacity var(--md-sys-motion-duration-medium1, 250ms) var(--md-sys-motion-easing-standard, cubic-bezier(0.2, 0, 0, 1)),
    transform var(--md-sys-motion-duration-medium1, 250ms) var(--md-sys-motion-easing-decelerate, cubic-bezier(0, 0, 0, 1));
}
.page-fade-enter-from {
  opacity: 0;
  transform: translateY(8px);
}
.page-fade-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
