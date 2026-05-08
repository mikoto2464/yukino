// ============================================================
// 路由配置
// 基于旧前端业务逻辑：public > auth > admin 三级权限
// 对接 /api/user/me 实现 session 恢复与角色判定
// ============================================================
import {
  createRouter,
  createWebHistory,
  type NavigationGuardNext,
  type RouteLocationNormalized,
} from "vue-router";
import { useAuthStore } from "@/stores/auth";
import { useFeedbackStore } from "@/stores/feedback";
import { fetchUserMe } from "@/api/services";

/** 路由 meta 中的访问级别 */
type AccessLevel = "public" | "auth" | "admin";

// 异步加载布局组件
const UserLayout = () => import("@/layouts/UserLayout.vue");
const AdminLayout = () => import("@/layouts/AdminLayout.vue");

// 异步加载页面
const PublicView = () => import("@/views/PublicView.vue");
const LoginView = () => import("@/views/LoginView.vue");
const Dashboard = () => import("@/views/user/Dashboard.vue");
const CDKeys = () => import("@/views/admin/CDKeys.vue");
const UsersManage = () => import("@/views/admin/UsersManage.vue");
const ProjectsManage = () => import("@/views/admin/ProjectsManage.vue");

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: UserLayout,
      children: [
        {
          path: "",
          name: "home",
          component: PublicView,
          meta: { access: "public" as AccessLevel },
        },
        {
          path: "login",
          name: "login",
          component: LoginView,
          meta: { access: "public" as AccessLevel },
        },
        {
          path: "dashboard",
          name: "dashboard",
          component: Dashboard,
          meta: { access: "auth" as AccessLevel },
        },
      ],
    },
    {
      path: "/admin",
      component: AdminLayout,
      children: [
        {
          path: "",
          name: "admin",
          component: CDKeys,
          meta: { access: "admin" as AccessLevel },
        },
        {
          path: "users",
          name: "admin-users",
          component: UsersManage,
          meta: { access: "admin" as AccessLevel },
        },
        {
          path: "projects",
          name: "admin-projects",
          component: ProjectsManage,
          meta: { access: "admin" as AccessLevel },
        },
      ],
    },
  ],
});

// ---------- 导航守卫 ----------

let sessionRefreshed = false;

async function refreshSession() {
  if (sessionRefreshed) return;
  sessionRefreshed = true;

  const authStore = useAuthStore();
  try {
    const user = await fetchUserMe(true);
    const role = String(user.role).toLowerCase() === "admin" ? "admin" : "user";
    authStore.setSessionUser({
      id: String(user.id),
      name: user.nickname || "User",
      roles: role === "admin" ? ["user", "admin"] : ["user"],
      nickname: user.nickname,
      avatarUrl: user.avatar_url,
      authStamp: user.auth_stamp,
    });
  } catch {
    // 未登录或接口失败时静默忽略
  }
}

router.beforeEach(async (to) => {
  const authStore = useAuthStore();
  const feedbackStore = useFeedbackStore();
  const access: AccessLevel = (to.meta.access as AccessLevel) ?? "public";

  if (access === "public") return true;

  // auth / admin 级页面需要先恢复 session
  await refreshSession();

  if (!authStore.isAuthenticated) {
    feedbackStore.open({
      type: "info",
      message: "请先登录后再访问该页面",
    });
    return { name: "login", query: { redirect: to.fullPath } };
  }

  if (access === "admin" && !authStore.isAdmin) {
    feedbackStore.open({
      type: "error",
      message: "当前账号缺少管理员权限",
    });
    return { name: "dashboard" };
  }

  return true;
});

export default router;
