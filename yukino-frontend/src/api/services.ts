// ============================================================
// 业务 Service 层
// 每个方法对应后端接口清单中的一个 API
// 对接后端 API 契约，所有返回均已被 http 层拆包
// ============================================================
import { get, post, del } from "./http";
import type {
  User,
  Device,
  Project,
  TelegramAuthPayload,
  CreateDevicePayload,
  RedemptionPayload,
} from "@/types";

// ---------- Auth ----------

/** POST /api/auth/session/telegram */
export function authSessionTelegram(payload: TelegramAuthPayload) {
  return post<User>("/api/auth/session/telegram", payload);
}

/** DELETE /api/auth/session */
export function deleteAuthSession() {
  return del<void>("/api/auth/session");
}

// ---------- User ----------

/** GET /api/user/me */
export function fetchUserMe(silent?: boolean) {
  return get<User>("/api/user/me", { silentError: silent });
}

// ---------- Device ----------

/** GET /api/devices */
export function fetchDevices() {
  return get<Device[]>("/api/devices");
}

/** POST /api/devices */
export function createDevice(payload: CreateDevicePayload) {
  return post<Device>("/api/devices", payload);
}

/** DELETE /api/devices/{hardware_id} */
export function deleteDevice(hardwareId: string) {
  return del<void>(`/api/devices/${encodeURIComponent(hardwareId)}`);
}

// ---------- Project ----------

/** GET /api/projects */
export function fetchProjects() {
  return get<Project[]>("/api/projects");
}

// ---------- Subscription ----------

/** POST /api/subscriptions/redemption */
export function redeemCdkey(payload: RedemptionPayload) {
  return post<void>("/api/subscriptions/redemption", payload);
}

// ============================================================
// TODO: 此处接口后端暂未实现，目前为占位逻辑，待后续对接
// ============================================================

/** [Mock] 封禁用户 */
export async function banUser(userId: string, durationHours: number) {
  // TODO: 此处接口后端暂未实现，目前为占位逻辑，待后续对接
  await new Promise((r) => setTimeout(r, 600));
  console.warn(`[Mock] banUser: userId=${userId}, hours=${durationHours}`);
}

/** [Mock] 强制踢下线设备 */
export async function kickDevice(deviceId: string) {
  // TODO: 此处接口后端暂未实现，目前为占位逻辑，待后续对接
  await new Promise((r) => setTimeout(r, 400));
  console.warn(`[Mock] kickDevice: deviceId=${deviceId}`);
}

/** [Mock] 修改用户设备上限 */
export async function updateUserMaxDevices(userId: string, maxDevices: number) {
  // TODO: 此处接口后端暂未实现，目前为占位逻辑，待后续对接
  await new Promise((r) => setTimeout(r, 450));
  console.warn(`[Mock] updateMaxDevices: userId=${userId}, max=${maxDevices}`);
}
