import axios, {AxiosError, type InternalAxiosRequestConfig} from 'axios'
import {useAuthStore} from '../stores/auth'
import {useFeedbackStore} from '../stores/feedback'

export interface ApiEnvelope<T> {
    data: T | null
    message: string
    timestamp: number
}

const http = axios.create({
    baseURL: '/api',
    timeout: 12000
})

function notifyError(message: string) {
    const feedbackStore = useFeedbackStore()
    feedbackStore.open({
        type: 'error',
        message
    })
}

http.interceptors.request.use((config: InternalAxiosRequestConfig) => {
    const authStore = useAuthStore()

    if (authStore.token) {
        config.headers.Authorization = `Bearer ${authStore.token}`
    }

    return config
})

http.interceptors.response.use(
    (response): any => {
        const payload = response.data as ApiEnvelope<unknown>

        if (!payload || typeof payload !== 'object' || !('data' in payload)) {
            const message = '响应格式异常'
            notifyError(message)
            return Promise.reject(new Error(message))
        }

        if (payload.data === null) {
            const message = payload.message || '业务请求失败'
            notifyError(message)
            return Promise.reject(new Error(message))
        }

        return payload.data
    },
    (error: AxiosError<ApiEnvelope<null>>) => {
        const backendMessage = error.response?.data?.message
        const message = backendMessage || error.message || '网络异常，请稍后重试'

        if (error.response?.status === 401) {
            const authStore = useAuthStore()
            authStore.clearAuth()
        }

        notifyError(message)
        return Promise.reject(error)
    }
)

export default http
