// ============================================================
// HTTP 客户端
// 封装 fetch，实现 YukinoResponse 拆包、Token 注入、错误统一通知
// 对接后端 API 契约：统一 YukinoResponse<T> 格式
// ============================================================
import type { YukinoResponse } from "@/types";

/** 扩展请求配置，允许调用方静默失败不弹 snackbar */
interface RequestConfig extends RequestInit {
  silentError?: boolean;
}

// ---------- 运行时依赖注入（store 在调用时延迟注入，避免循环依赖） ----------
let _getToken: (() => string) | null = null;
let _clearAuth: (() => void) | null = null;
let _notifyError: ((msg: string) => void) | null = null;

export function installAuthProvider(getToken: () => string) {
  _getToken = getToken;
}

export function installClearAuth(fn: () => void) {
  _clearAuth = fn;
}

export function installErrorNotifier(fn: (msg: string) => void) {
  _notifyError = fn;
}

// ---------- 核心请求方法 ----------

async function request<T>(
  method: string,
  url: string,
  body?: unknown,
  config?: RequestConfig,
): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };

  const token = _getToken?.();
  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  try {
    const resp = await fetch(url, {
      method,
      headers,
      body: body != null ? JSON.stringify(body) : undefined,
      signal: config?.signal,
      credentials: "same-origin",
    });

    // 401 → 清空本地认证态
    if (resp.status === 401) {
      _clearAuth?.();
    }

    // 尝试解析为 JSON
    const contentType = resp.headers.get("content-type") ?? "";
    if (!contentType.includes("application/json")) {
      throw new Error(`响应格式异常 (${resp.status})`);
    }

    const payload: YukinoResponse<T> = await resp.json();

    // 按 YukinoResponse 拆包
    if (!payload || typeof payload !== "object" || !("data" in payload)) {
      const msg = "响应格式异常";
      if (!config?.silentError) _notifyError?.(msg);
      throw new Error(msg);
    }

    if (payload.data === null || payload.data === undefined) {
      const msg = payload.message || "业务请求失败";
      if (!config?.silentError) _notifyError?.(msg);
      throw new Error(msg);
    }

    return payload.data as T;
  } catch (err: unknown) {
    // 网络层异常
    if (
      err instanceof TypeError ||
      (err instanceof Error && err.name === "AbortError")
    ) {
      if (!config?.silentError) _notifyError?.("网络异常，请稍后重试");
      throw err;
    }
    // 已处理过的业务异常直接抛
    throw err;
  }
}

// ---------- 公开方法 ----------

export function get<T>(url: string, config?: RequestConfig) {
  return request<T>("GET", url, undefined, config);
}

export function post<T>(url: string, body?: unknown, config?: RequestConfig) {
  return request<T>("POST", url, body, config);
}

export function del<T>(url: string, config?: RequestConfig) {
  return request<T>("DELETE", url, undefined, config);
}
