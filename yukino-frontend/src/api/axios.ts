import axios, {AxiosError, type InternalAxiosRequestConfig} from 'axios'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

export interface ApiEnvelope<T> {
    data: T | null
    message: string
    timestamp: number
}

interface RequestConfigWithSilent extends InternalAxiosRequestConfig {
    silentError?: boolean
}

const http = axios.create({
    baseURL: '/api',
    timeout: 12000,
    withCredentials: true
})

function notifyError(message: string) {
    const feedbackStore = useFeedbackStore()
    feedbackStore.open({
        type: 'error',
        message
    })
}

http.interceptors.request.use((config: RequestConfigWithSilent) => {
    const authStore = useAuthStore()

    if (authStore.token) {
        config.headers.Authorization = `Bearer ${authStore.token}`
    }

    return config
})

http.interceptors.response.use(
    (response): any => {
        const config = response.config as RequestConfigWithSilent
        const silentError = Boolean(config.silentError)
        const payload = response.data as ApiEnvelope<unknown>

        if (!payload || typeof payload !== 'object' || !('data' in payload)) {
            const message = '响应格式异常'
            if (!silentError) {
                notifyError(message)
            }
            return Promise.reject(new Error(message))
        }

        if (payload.data === null) {
            const message = payload.message || '业务请求失败'
            if (!silentError) {
                notifyError(message)
            }
            return Promise.reject(new Error(message))
        }

        return payload.data
    },
    (error: AxiosError<ApiEnvelope<null>>) => {
        const config = error.config as RequestConfigWithSilent | undefined
        const silentError = Boolean(config?.silentError)
        const backendMessage = error.response?.data?.message
        const message = backendMessage || error.message || '网络异常，请稍后重试'

        if (error.response?.status === 401) {
            const authStore = useAuthStore()
            authStore.clearAuth()
        }

        if (!silentError) {
            notifyError(message)
        }
        return Promise.reject(error)
    }
)

export default http
