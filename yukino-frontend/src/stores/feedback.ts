import { defineStore } from 'pinia'

export type SnackbarType = 'success' | 'error' | 'info'

interface SnackbarPayload {
  message: string
  type?: SnackbarType
  timeout?: number
}

interface FeedbackState {
  visible: boolean
  message: string
  type: SnackbarType
  timeout: number
}

export const useFeedbackStore = defineStore('feedback', {
  state: (): FeedbackState => ({
    visible: false,
    message: '',
    type: 'info',
    timeout: 2800
  }),
  actions: {
    open(payload: SnackbarPayload) {
      this.message = payload.message
      this.type = payload.type ?? 'info'
      this.timeout = payload.timeout ?? 2800
      this.visible = true
    },
    close() {
      this.visible = false
    }
  }
})
