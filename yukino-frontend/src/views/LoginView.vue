<template>
  <div class="login-page">
    <div class="login-card card-surface">
      <h1 class="page-title">登录</h1>
      <p class="section-subtitle" style="margin-top: 4px">
        选择一种方式登录 Yukino
      </p>

      <div class="login-methods">
        <!-- Telegram 登录 -->
        <button
          class="btn btn-primary login-btn"
          :disabled="loadingTelegram"
          @click="loginByTelegram"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            style="flex-shrink: 0"
          >
            <path
              d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm5.562 8.161c-.18 1.897-.96 6.504-1.357 8.63-.168.896-.498 1.197-.819 1.226-.696.064-1.225-.46-1.9-.902-1.056-.692-1.653-1.123-2.678-1.799-1.185-.78-.417-1.21.258-1.911.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.479.329-.913.489-1.302.481-.428-.009-1.252-.242-1.865-.441-.751-.244-1.349-.374-1.297-.789.027-.216.324-.437.893-.662 3.498-1.524 5.83-2.529 6.998-3.015 3.332-1.386 4.025-1.627 4.476-1.635.099-.002.321.023.465.14.12.098.153.239.169.335.016.096.036.312.02.481z"
              fill="currentColor"
            />
          </svg>
          {{ loadingTelegram ? "登录中…" : "使用 Telegram 登录" }}
        </button>

        <!-- QQ 登录 (Mock) -->
        <button
          class="btn btn-tonal login-btn"
          :disabled="loadingQQ"
          @click="loginByQQ"
        >
          {{ loadingQQ ? "登录中…" : "使用 QQ 登录" }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { useFeedbackStore } from "@/stores/feedback";
import { authSessionTelegram } from "@/api/services";
import type { TelegramAuthPayload } from "@/types";

const router = useRouter();
const route = useRoute();
const authStore = useAuthStore();
const feedbackStore = useFeedbackStore();

const loadingTelegram = ref(false);
const loadingQQ = ref(false);
const botId = "8691993144";

// 加载 Telegram 登录 Widget 脚本
onMounted(() => {
  if (!document.getElementById("telegram-widget-script")) {
    const script = document.createElement("script");
    script.id = "telegram-widget-script";
    script.src = "https://telegram.org/js/telegram-widget.js?22";
    script.async = true;
    document.head.appendChild(script);
  }
});

/** 登录成功后跳转到目标页面 */
function doneRedirect() {
  const redirect = route.query.redirect;
  if (typeof redirect === "string" && redirect.length > 0) {
    router.push(redirect);
    return;
  }
  router.push({ name: "dashboard" });
}

/** Telegram 登录流程 */
function loginByTelegram() {
  const tg = (window as unknown as Record<string, unknown>).Telegram as
    | { Login?: { auth: (opts: object, cb: (data: unknown) => void) => void } }
    | undefined;

  if (!tg?.Login?.auth) {
    feedbackStore.open({
      type: "info",
      message: "Telegram 组件尚未加载完成，请稍后再试",
    });
    return;
  }

  loadingTelegram.value = true;

  tg.Login.auth({ bot_id: botId, request_access: "write" }, async (rawData) => {
    loadingTelegram.value = false;

    if (!rawData) {
      feedbackStore.open({ type: "info", message: "用户取消了授权" });
      return;
    }

    try {
      const user = await authSessionTelegram(rawData as TelegramAuthPayload);
      const role =
        String(user.role).toLowerCase() === "admin" ? "admin" : "user";

      authStore.setSessionUser({
        id: String(user.id),
        name: user.nickname || "User",
        roles: role === "admin" ? ["user", "admin"] : ["user"],
        nickname: user.nickname,
        avatarUrl: user.avatar_url,
        authStamp: user.auth_stamp,
      });

      feedbackStore.open({ type: "success", message: "登录成功" });
      doneRedirect();
    } catch {
      // 错误已由 HTTP 层处理
    }
  });
}

/** QQ Mock 登录 — 开发状态下使用 */
function loginByQQ() {
  loadingQQ.value = true;
  setTimeout(() => {
    authStore.setSessionUser({
      id: "2",
      name: "Yukino User",
      roles: ["user"],
      nickname: "Yukino User",
    });
    feedbackStore.open({
      type: "success",
      message: "已登录为普通用户（Mock）",
    });
    loadingQQ.value = false;
    doneRedirect();
  }, 500);
}
</script>

<style scoped>
.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 112px);
  padding: 24px;
}

.login-card {
  width: 100%;
  max-width: 400px;
}

.login-methods {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 24px;
}

.login-btn {
  width: 100%;
  padding: 14px 24px;
  font-size: 0.9375rem;
}
</style>
