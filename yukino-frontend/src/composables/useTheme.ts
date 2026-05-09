// ============================================================
// 主题状态管理 Composable
// 控制深浅模式、壁纸选择、色彩引擎调度
// 对外暴露所有主题相关操作
// ============================================================
import { reactive, readonly } from "vue";
import {
  extractThemeFromImage,
  applyColorScheme,
  type ThemeSchemeResult,
} from "@/theme/colorEngine";
import {
  BACKGROUND_OPTIONS,
  BG_OPACITY,
  BG_MASK_OPACITY,
  THEME_CACHE_PREFIX,
  type BackgroundOption,
} from "@/config";

export type ThemeMode = "system" | "light" | "dark";

const STORAGE_MODE = "yukino.theme.mode";
const STORAGE_BG = "yukino.theme.background";

interface ThemeState {
  mode: ThemeMode;
  background: string;
  ready: boolean;
}

const state = reactive<ThemeState>({
  mode: "system",
  background: BACKGROUND_OPTIONS[0]?.key ?? "",
  ready: false,
});

let darkMediaQuery: MediaQueryList | null = null;
let initToken = 0;

// ---------- 工具函数 ----------

/** 获取系统实际偏好 */
function systemDark(): boolean {
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

/** 解析当前生效的模式名 */
export function effectiveDark(): boolean {
  return state.mode === "system" ? systemDark() : state.mode === "dark";
}

/** 获取当前壁纸配置 */
function currentBg(): BackgroundOption | undefined {
  return BACKGROUND_OPTIONS.find((b) => b.key === state.background);
}

// ---------- localStorage 缓存 ----------

function readCachedColors(bgKey: string): ThemeSchemeResult | null {
  try {
    const raw = localStorage.getItem(`${THEME_CACHE_PREFIX}.${bgKey}`);
    if (!raw) return null;
    return JSON.parse(raw) as ThemeSchemeResult;
  } catch {
    return null;
  }
}

function writeCachedColors(bgKey: string, colors: ThemeSchemeResult) {
  try {
    localStorage.setItem(
      `${THEME_CACHE_PREFIX}.${bgKey}`,
      JSON.stringify(colors),
    );
  } catch {
    // localStorage 满则丢弃
  }
}

function readStorage() {
  const mode = localStorage.getItem(STORAGE_MODE);
  if (mode === "system" || mode === "light" || mode === "dark") {
    state.mode = mode;
  }
  const bg = localStorage.getItem(STORAGE_BG);
  if (bg && BACKGROUND_OPTIONS.some((b) => b.key === bg)) {
    state.background = bg;
  }
}

function writeStorage() {
  localStorage.setItem(STORAGE_MODE, state.mode);
  localStorage.setItem(STORAGE_BG, state.background);
}

// ---------- 应用壁纸背景 ----------

function applyBackgroundStyle(dark: boolean) {
  const bg = currentBg();
  if (!bg) return;
  document.documentElement.style.setProperty(
    "--yukino-bg-image",
    `url("${bg.url}")`,
  );
  document.documentElement.style.setProperty(
    "--yukino-bg-opacity",
    String(dark ? BG_OPACITY.dark : BG_OPACITY.light),
  );
  document.documentElement.style.setProperty(
    "--yukino-bg-mask-opacity",
    String(dark ? BG_MASK_OPACITY.dark : BG_MASK_OPACITY.light),
  );
}

// ---------- 核心主题应用 ----------

const colorSchemeCache = new Map<string, ThemeSchemeResult>();

async function applyTheme(recompute = false) {
  const token = ++initToken;
  const bg = currentBg();
  if (!bg) return;

  let colors = colorSchemeCache.get(bg.key) ?? readCachedColors(bg.key);
  const needExtract = recompute || !colors;

  if (needExtract) {
    try {
      colors = await extractThemeFromImage(bg.url);
      colorSchemeCache.set(bg.key, colors);
      writeCachedColors(bg.key, colors);
    } catch {
      colors = readCachedColors(bg.key) ?? colorSchemeCache.get(bg.key) ?? null;
    }
  }

  if (token !== initToken || !colors) return;

  const dark = effectiveDark();
  applyColorScheme(dark ? colors.dark : colors.light);
  applyBackgroundStyle(dark);

  document.documentElement.classList.toggle("dark", dark);
  document.documentElement.classList.toggle("light", !dark);
}

/** 系统主题变更回调 */
function onSystemChange() {
  if (state.mode === "system") applyTheme(false);
}

// ---------- 公开 Composable ----------

export function useTheme() {
  /** 初始化：读取存储→提取色彩→挂载系统监听 */
  async function init() {
    if (state.ready) return;
    readStorage();
    await applyTheme(true);
    state.ready = true;

    darkMediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    darkMediaQuery.addEventListener("change", onSystemChange);
  }

  function setMode(mode: ThemeMode) {
    state.mode = mode;
    writeStorage();
    applyTheme(false);
  }

  function setBackground(bgKey: string) {
    if (!BACKGROUND_OPTIONS.some((b) => b.key === bgKey)) return;
    state.background = bgKey;
    writeStorage();
    applyTheme(true);
  }

  function setRandomBackground() {
    const others = BACKGROUND_OPTIONS.filter((b) => b.key !== state.background);
    const pick =
      others.length > 0
        ? others[Math.floor(Math.random() * others.length)]!
        : BACKGROUND_OPTIONS[0]!;
    setBackground(pick.key);
  }

  function toggleDark() {
    setMode(effectiveDark() ? "light" : "dark");
  }

  return {
    state: readonly(state),
    init,
    setMode,
    setBackground,
    setRandomBackground,
    toggleDark,
    effectiveDark,
  };
}
