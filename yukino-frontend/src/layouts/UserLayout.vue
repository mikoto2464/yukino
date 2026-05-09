<template>
  <div class="user-layout">
    <MdTopAppBar>
      <template #start>
        <RouterLink to="/" class="brand-link">Yukino</RouterLink>
        <nav class="top-bar-nav">
          <RouterLink
            v-if="auth.isAdmin"
            to="/admin"
            class="nav-link"
          >
            管理后台
          </RouterLink>
        </nav>
      </template>
      <template #end>
        <ThemeControls />
        <MdButton
          v-if="auth.isAuthenticated"
          variant="text"
          :disabled="logoutLoading"
          @click="handleLogout"
        >
          退出登录
        </MdButton>
      </template>
    </MdTopAppBar>

    <main class="main-content">
      <RouterView />
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
.user-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.brand-link {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--md-sys-color-on-surface, #1d1b20);
  letter-spacing: 0.01em;
  text-decoration: none;
}

.top-bar-nav {
  display: flex;
  gap: 4px;
  margin-left: 16px;
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
.nav-link.router-link-active {
  background: var(--md-sys-color-secondary-container, #e8def8);
  color: var(--md-sys-color-on-secondary-container, #1d192b);
}

.main-content {
  flex: 1;
  padding: 24px;
  max-width: 1400px;
  width: 100%;
  margin: 0 auto;
}
</style>
