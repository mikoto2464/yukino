// ============================================================
// 应用入口
// 顺序：主题初始化 → Pinia → Router → 挂载
// 保证 CSS 变量在 Vue 组件渲染前就绪
// ============================================================
import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "@/App.vue";
import router from "@/router";
import { useTheme } from "@/composables/useTheme";
import {
  installAuthProvider,
  installClearAuth,
  installErrorNotifier,
} from "@/api/http";
import { useAuthStore } from "@/stores/auth";
import { useFeedbackStore } from "@/stores/feedback";
import "@/styles/main.css";

// ---- 启动计时：保证 loading 动画至少展示 950ms ----
const bootStart = performance.now();
const MIN_BOOT_MS = 950;

async function boot() {
  // 1. 初始化 MD3 主题（取色壁纸 → 注入 CSS 变量）
  const { init } = useTheme();
  await init();

  // 2. 创建应用
  const app = createApp(App);
  const pinia = createPinia();
  app.use(pinia);
  app.use(router);

  // 3. 挂载后再注入 Store 依赖到 HTTP 层（避免循环依赖）
  app.mount("#app");

  const authStore = useAuthStore();
  const feedbackStore = useFeedbackStore();

  installAuthProvider(() => authStore.token);
  installClearAuth(() => authStore.clearAuth());
  installErrorNotifier((msg: string) =>
    feedbackStore.open({ type: "error", message: msg }),
  );

  // 4. 隐藏启动屏
  const spent = performance.now() - bootStart;
  const wait = Math.max(0, MIN_BOOT_MS - spent);
  setTimeout(() => {
    const el = document.getElementById("boot-loading");
    if (el) {
      el.classList.add("hidden");
      setTimeout(() => el.remove(), 420);
    }
  }, wait);
}

void boot();
