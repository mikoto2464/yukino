// ============================================================
// 全局反馈 Store
// 控制 snackbar 的显隐、消息内容和类型
// ============================================================
import { defineStore } from "pinia";
import { ref } from "vue";
import type { SnackbarType } from "@/types";

interface SnackbarPayload {
  message: string;
  type?: SnackbarType;
  timeout?: number;
}

export const useFeedbackStore = defineStore("feedback", () => {
  const visible = ref(false);
  const message = ref("");
  const type = ref<SnackbarType>("info");
  const timeout = ref(2800);

  function open(payload: SnackbarPayload) {
    message.value = payload.message;
    type.value = payload.type ?? "info";
    timeout.value = payload.timeout ?? 2800;
    visible.value = true;
  }

  function close() {
    visible.value = false;
  }

  return { visible, message, type, timeout, open, close };
});
