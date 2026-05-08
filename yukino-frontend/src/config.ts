// ============================================================
// 全局配置入口
// 集中管理壁纸、默认主题色、布局间距等可调节参数
// 修改尺寸/颜色/间距等均在此文件中调整
// ============================================================

/** 浅色模式下内容容器的最大宽度（px） */
export const CONTENT_MAX_WIDTH = 1400;

/** 浅色模式下侧边栏宽度（px） */
export const DRAWER_WIDTH = 260;

/** 全局过渡动画时长（ms） — 控制主题切换、菜单展开等 */
export const TRANSITION_DURATION = 360;

/** MD3 默认种子色（HSL 色相），取色引擎失败时使用 */
export const FALLBACK_SEED_HUE = 210;

/** 壁纸透明度：深色 / 浅色模式 */
export const BG_OPACITY = { dark: 0.24, light: 0.38 };

/** 深色/浅色壁纸覆盖层的遮罩透明度 */
export const BG_MASK_OPACITY = { dark: 0.62, light: 0.52 };

// ============================================================
// 背景壁纸注册表
// 文件名对应 public/background/ 目录中的图片
// ============================================================
export interface BackgroundOption {
  key: string;
  label: string;
  url: string;
  thumbUrl: string;
}

const BG_NAMES = [
  "119391638_p0.png",
  "119812287_p0.png",
  "121052377_p0.png",
  "122128866_p0.png",
  "124263276_p0.png",
  "125245434_p0.png",
  "127133721_p0.png",
  "127441961_p0.png",
  "128560882_p0.png",
  "130143139_p0.png",
  "136261196_p0.png",
  "137874580_p0.png",
  "138900181_p0.png",
  "139030627_p0.png",
];

export const BACKGROUND_OPTIONS: BackgroundOption[] = BG_NAMES.map(
  (name, i) => {
    const base = name.replace(".png", "");
    return {
      key: name,
      label: `背景 ${i + 1}`,
      url: `/background/${name}`,
      thumbUrl: `/background/thumbs/${base}.jpg`,
    };
  },
);

/** 取 localStorage 中缓存的色彩 key 前缀 */
export const THEME_CACHE_PREFIX = "yukino.theme.cached";
