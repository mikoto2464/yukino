import {createRouter, createWebHistory} from 'vue-router'
import NProgress from 'nprogress'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

import UserLayout from '../layouts/UserLayout.vue'
import AdminLayout from '../layouts/AdminLayout.vue'
import PublicView from '../views/PublicView.vue'
import LoginView from '../views/LoginView.vue'
import DashboardView from '../views/user/Dashboard.vue'
import CDKeysView from '../views/admin/CDKeys.vue'
import ProjectsManageView from '../views/admin/ProjectsManage.vue'
import UsersManageView from '../views/admin/UsersManage.vue'

NProgress.configure({
    showSpinner: false,
    trickleSpeed: 120
})

type AccessLevel = 'public' | 'auth' | 'admin'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: UserLayout,
            children: [
                {
                    path: '',
                    name: 'home',
                    component: PublicView,
                    meta: {access: 'public' as AccessLevel}
                },
                {
                    path: 'login',
                    name: 'login',
                    component: LoginView,
                    meta: {access: 'public' as AccessLevel}
                },
                {
                    path: 'dashboard',
                    name: 'dashboard',
                    component: DashboardView,
                    meta: {access: 'auth' as AccessLevel}
                }
            ]
        },
        {
            path: '/admin',
            component: AdminLayout,
            children: [
                {
                    path: '',
                    name: 'admin',
                    component: CDKeysView,
                    meta: {access: 'admin' as AccessLevel}
                },
                {
                    path: 'users',
                    name: 'admin-users',
                    component: UsersManageView,
                    meta: {access: 'admin' as AccessLevel}
                },
                {
                    path: 'projects',
                    name: 'admin-projects',
                    component: ProjectsManageView,
                    meta: {access: 'admin' as AccessLevel}
                }
            ]
        }
    ]
})

router.beforeEach((to) => {
    NProgress.start()

    const authStore = useAuthStore()
    const feedbackStore = useFeedbackStore()
    const access = (to.meta.access as AccessLevel | undefined) ?? 'public'

    if (access === 'public') {
        return true
    }

    if (!authStore.isAuthenticated) {
        feedbackStore.open({
            type: 'info',
            message: '请先登录后再访问该页面'
        })

        return {
            name: 'login',
            query: {
                redirect: to.fullPath
            }
        }
    }

    if (access === 'admin' && !authStore.isAdmin) {
        feedbackStore.open({
            type: 'error',
            message: '当前账号缺少管理员权限'
        })

        return {
            name: 'dashboard'
        }
    }

    return true
})

router.afterEach(() => {
    NProgress.done()
})

router.onError(() => {
    NProgress.done()
})

export default router
