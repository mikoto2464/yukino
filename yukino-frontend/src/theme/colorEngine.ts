// ============================================================
// MD3 色彩引擎核心
// 利用 @material/material-color-utilities 从壁纸提取 Monet 色彩
// 生成 Tonal Palette → CSS 变量，覆盖 @material/web 默认色
// 修改色板映射或 Token 定义请修改 TOKEN_DEFS 数组
// ============================================================
import {
  Hct,
  QuantizerCelebi,
  Score,
  argbFromRgb,
  SchemeTonalSpot,
  type TonalPalette,
} from "@material/material-color-utilities";

/** ARGB 整型 → CSS #rrggbb */
function argbToHex(argb: number): string {
  const r = (argb >> 16) & 0xff;
  const g = (argb >> 8) & 0xff;
  const b = argb & 0xff;
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

/** 从 TonalPalette 提取指定 tone 的 hex */
function toneHex(palette: TonalPalette, tone: number): string {
  return argbToHex(palette.getHct(tone).toInt());
}

// ---------- 色彩 Token 定义 ----------

interface TokenDef {
  cssVar: string;
  palette: PaletteKey;
  lightTone: number;
  darkTone: number;
}

const PALETTE_KEYS = [
  "primaryPalette",
  "secondaryPalette",
  "tertiaryPalette",
  "neutralPalette",
  "neutralVariantPalette",
  "errorPalette",
] as const;

type PaletteKey = (typeof PALETTE_KEYS)[number];

/** MD3 全部 CSS Token 定义 — 浅色/深色分别取不同 tone */
const TOKEN_DEFS: TokenDef[] = [
  // Primary
  {
    cssVar: "--md-sys-color-primary",
    palette: "primaryPalette",
    lightTone: 40,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-on-primary",
    palette: "primaryPalette",
    lightTone: 100,
    darkTone: 20,
  },
  {
    cssVar: "--md-sys-color-primary-container",
    palette: "primaryPalette",
    lightTone: 90,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-on-primary-container",
    palette: "primaryPalette",
    lightTone: 10,
    darkTone: 90,
  },
  // Secondary
  {
    cssVar: "--md-sys-color-secondary",
    palette: "secondaryPalette",
    lightTone: 40,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-on-secondary",
    palette: "secondaryPalette",
    lightTone: 100,
    darkTone: 20,
  },
  {
    cssVar: "--md-sys-color-secondary-container",
    palette: "secondaryPalette",
    lightTone: 90,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-on-secondary-container",
    palette: "secondaryPalette",
    lightTone: 10,
    darkTone: 90,
  },
  // Tertiary
  {
    cssVar: "--md-sys-color-tertiary",
    palette: "tertiaryPalette",
    lightTone: 40,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-on-tertiary",
    palette: "tertiaryPalette",
    lightTone: 100,
    darkTone: 20,
  },
  {
    cssVar: "--md-sys-color-tertiary-container",
    palette: "tertiaryPalette",
    lightTone: 90,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-on-tertiary-container",
    palette: "tertiaryPalette",
    lightTone: 10,
    darkTone: 90,
  },
  // Error
  {
    cssVar: "--md-sys-color-error",
    palette: "errorPalette",
    lightTone: 40,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-on-error",
    palette: "errorPalette",
    lightTone: 100,
    darkTone: 20,
  },
  {
    cssVar: "--md-sys-color-error-container",
    palette: "errorPalette",
    lightTone: 90,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-on-error-container",
    palette: "errorPalette",
    lightTone: 10,
    darkTone: 90,
  },
  // Surface / Background
  {
    cssVar: "--md-sys-color-background",
    palette: "neutralPalette",
    lightTone: 99,
    darkTone: 6,
  },
  {
    cssVar: "--md-sys-color-on-background",
    palette: "neutralPalette",
    lightTone: 10,
    darkTone: 90,
  },
  {
    cssVar: "--md-sys-color-surface",
    palette: "neutralPalette",
    lightTone: 99,
    darkTone: 6,
  },
  {
    cssVar: "--md-sys-color-on-surface",
    palette: "neutralPalette",
    lightTone: 10,
    darkTone: 90,
  },
  {
    cssVar: "--md-sys-color-surface-variant",
    palette: "neutralVariantPalette",
    lightTone: 90,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-on-surface-variant",
    palette: "neutralVariantPalette",
    lightTone: 30,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-outline",
    palette: "neutralVariantPalette",
    lightTone: 50,
    darkTone: 60,
  },
  {
    cssVar: "--md-sys-color-outline-variant",
    palette: "neutralVariantPalette",
    lightTone: 80,
    darkTone: 30,
  },
  {
    cssVar: "--md-sys-color-surface-tint",
    palette: "primaryPalette",
    lightTone: 40,
    darkTone: 80,
  },
  {
    cssVar: "--md-sys-color-shadow",
    palette: "neutralPalette",
    lightTone: 0,
    darkTone: 0,
  },
  {
    cssVar: "--md-sys-color-scrim",
    palette: "neutralPalette",
    lightTone: 0,
    darkTone: 0,
  },
  {
    cssVar: "--md-sys-color-inverse-surface",
    palette: "neutralPalette",
    lightTone: 20,
    darkTone: 90,
  },
  {
    cssVar: "--md-sys-color-inverse-on-surface",
    palette: "neutralPalette",
    lightTone: 95,
    darkTone: 20,
  },
  {
    cssVar: "--md-sys-color-inverse-primary",
    palette: "primaryPalette",
    lightTone: 80,
    darkTone: 40,
  },
  // Surface containers (MD3 elevation 层级)
  {
    cssVar: "--md-sys-color-surface-container-lowest",
    palette: "neutralPalette",
    lightTone: 100,
    darkTone: 4,
  },
  {
    cssVar: "--md-sys-color-surface-container-low",
    palette: "neutralPalette",
    lightTone: 96,
    darkTone: 10,
  },
  {
    cssVar: "--md-sys-color-surface-container",
    palette: "neutralPalette",
    lightTone: 94,
    darkTone: 12,
  },
  {
    cssVar: "--md-sys-color-surface-container-high",
    palette: "neutralPalette",
    lightTone: 92,
    darkTone: 17,
  },
  {
    cssVar: "--md-sys-color-surface-container-highest",
    palette: "neutralPalette",
    lightTone: 90,
    darkTone: 22,
  },
];

/** 从 SchemeTonalSpot 实例提取所有 CSS 变量 */
function extractSchemeVars(
  scheme: SchemeTonalSpot,
  isDark: boolean,
): Record<string, string> {
  const result: Record<string, string> = {};
  const palettes = scheme as unknown as Record<string, TonalPalette>;
  for (const def of TOKEN_DEFS) {
    const palette = palettes[def.palette];
    if (!palette) continue;
    const tone = isDark ? def.darkTone : def.lightTone;
    result[def.cssVar] = toneHex(palette, tone);
  }
  return result;
}

// ---------- 像素取色 ----------

function extractSeedFromPixels(pixels: Uint8ClampedArray): number {
  const colors: number[] = [];
  for (let i = 0; i < pixels.length; i += 4) {
    const r = pixels[i]!;
    const g = pixels[i + 1]!;
    const b = pixels[i + 2]!;
    const a = pixels[i + 3]!;
    if (a < 127) continue;
    colors.push(argbFromRgb(r, g, b));
  }
  if (colors.length === 0) return argbFromRgb(100, 130, 180);

  const raw = QuantizerCelebi.quantize(colors, 128);
  const ranked = Score.score(raw);
  return ranked[0] ?? argbFromRgb(100, 130, 180);
}

// ---------- 公开 API ----------

export interface ThemeSchemeResult {
  light: Record<string, string>;
  dark: Record<string, string>;
}

const inMemoryCache = new Map<string, ThemeSchemeResult>();

/** 加载壁纸图片并提取 MD3 浅色+深色两套 CSS 变量 */
export async function extractThemeFromImage(
  imageUrl: string,
): Promise<ThemeSchemeResult> {
  const cached = inMemoryCache.get(imageUrl);
  if (cached) return cached;

  const img = await new Promise<HTMLImageElement>((resolve, reject) => {
    const el = new Image();
    el.crossOrigin = "anonymous";
    el.onload = () => resolve(el);
    el.onerror = () => reject(new Error(`无法加载图片: ${imageUrl}`));
    el.src = imageUrl;
  });

  const canvas = document.createElement("canvas");
  canvas.width = 56;
  canvas.height = 56;
  const ctx = canvas.getContext("2d", { willReadFrequently: true });
  if (!ctx) throw new Error("Canvas 不可用");

  ctx.drawImage(img, 0, 0, 56, 56);
  const pixels = ctx.getImageData(0, 0, 56, 56).data;
  const seedArgb = extractSeedFromPixels(pixels);
  const seedHct = Hct.fromInt(seedArgb);

  const lightScheme = new SchemeTonalSpot(seedHct, false, 0);
  const darkScheme = new SchemeTonalSpot(seedHct, true, 0);

  const result: ThemeSchemeResult = {
    light: extractSchemeVars(lightScheme, false),
    dark: extractSchemeVars(darkScheme, true),
  };

  inMemoryCache.set(imageUrl, result);
  return result;
}

/** 将 CSS 变量注入到 :root */
export function applyColorScheme(colors: Record<string, string>): void {
  const root = document.documentElement;
  for (const [key, value] of Object.entries(colors)) {
    root.style.setProperty(key, value);
  }
}
