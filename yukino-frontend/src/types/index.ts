// ============================================================
// 后端 API 契约对应的 TypeScript 类型定义
// 所有接口返回均包裹在 YukinoResponse 中
// ============================================================

/** 后端统一响应包装 */
export interface YukinoResponse<T = unknown> {
  data?: T;
  message: string;
  timestamp: number;
}

/** 核心数据模型 */
export interface User {
  id: number;
  nickname: string;
  avatar_url: string;
  /** 后端角色字段，可能是字符串或数字 */
  role: string | number;
  auth_stamp: string;
}

export interface Device {
  hardware_id: string;
  user_id: number;
  name: string;
  last_seen: number;
}

export interface Project {
  id: number;
  name: string;
}

// ============================================================
// 前端内部使用的用户角色类型
// ============================================================
export type UserRole = "user" | "admin";

/** 前端认证用户模型（从 /user/me 反序列化后统一角色格式） */
export interface AuthUser {
  id: string;
  name: string;
  roles: UserRole[];
  nickname?: string;
  avatarUrl?: string;
  authStamp?: string;
}

/** 反馈消息类型 */
export type SnackbarType = "success" | "error" | "info";

// ============================================================
// 请求 Payload 类型
// ============================================================

/** POST /api/auth/session/telegram */
export interface TelegramAuthPayload {
  id: number;
  first_name: string;
  last_name: string;
  username: string;
  photo_url: string;
  hash: string;
  auth_date: number;
}

/** POST /api/devices */
export interface CreateDevicePayload {
  hardware_id: string;
  name: string;
}

/** POST /api/subscriptions/redemption */
export interface RedemptionPayload {
  cdkey: string;
}
