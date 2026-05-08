// ============================================================
// 认证 Store
// 管理用户 token、基本信息、角色，持久化到 localStorage
// 对接 /api/auth/session/telegram 和 /api/user/me
// ============================================================
import { defineStore } from "pinia";
import { computed, ref } from "vue";
import type { AuthUser, UserRole } from "@/types";

const TOKEN_KEY = "yukino.token";
const USER_KEY = "yukino.user";

function readToken(): string {
  return localStorage.getItem(TOKEN_KEY) ?? "";
}

function readUser(): AuthUser | null {
  const raw = localStorage.getItem(USER_KEY);
  if (!raw) return null;
  try {
    return JSON.parse(raw) as AuthUser;
  } catch {
    localStorage.removeItem(USER_KEY);
    return null;
  }
}

export const useAuthStore = defineStore("auth", () => {
  const token = ref<string>(readToken());
  const user = ref<AuthUser | null>(readUser());

  const isAuthenticated = computed(() => Boolean(user.value || token.value));
  const isAdmin = computed(() => user.value?.roles.includes("admin") ?? false);
  const roles = computed<UserRole[]>(() => user.value?.roles ?? []);

  /** 直接设置 token（Bearer 方式由 http 层拼接） */
  function setToken(t: string) {
    token.value = t;
    if (t) {
      localStorage.setItem(TOKEN_KEY, t);
    } else {
      localStorage.removeItem(TOKEN_KEY);
    }
  }

  function setUser(u: AuthUser | null) {
    user.value = u;
    if (u) {
      localStorage.setItem(USER_KEY, JSON.stringify(u));
    } else {
      localStorage.removeItem(USER_KEY);
    }
  }

  /** Session 模式：token 为空，仅凭 Cookie/Session 维持 */
  function setSessionUser(u: AuthUser) {
    setToken("");
    setUser(u);
  }

  /** 完整清理 */
  function clearAuth() {
    token.value = "";
    user.value = null;
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(USER_KEY);
  }

  return {
    token,
    user,
    isAuthenticated,
    isAdmin,
    roles,
    setToken,
    setUser,
    setSessionUser,
    clearAuth,
  };
});
