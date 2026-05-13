/*
 * ============================================================
 * MD3 主题切换圆形扩散动画
 * 复刻 Android Activity Transition circular reveal
 * 从触发按钮中心扩散新背景色覆盖全页面
 * ============================================================
 */

/** 计算从圆心到视口最远角的距离 */
function maxRadius(cx: number, cy: number): number {
  const w = window.innerWidth;
  const h = window.innerHeight;
  const corners: [number, number][] = [
    [0, 0],
    [w, 0],
    [0, h],
    [w, h],
  ];
  return Math.max(
    ...corners.map(([x, y]) =>
      Math.sqrt((x - cx) ** 2 + (y - cy) ** 2),
    ),
  );
}

/**
 * 执行圆形扩散动画
 * @param trigger 触发按钮的 DOM 元素，取其中心作为扩散原点
 * @param newBackgroundHex 新主题的背景色 (如 #fef7ff)
 */
export function animateThemeTransition(
  trigger: HTMLElement,
  newBackgroundHex: string,
): Promise<void> {
  return new Promise((resolve) => {
    const rect = trigger.getBoundingClientRect();
    const cx = rect.left + rect.width / 2;
    const cy = rect.top + rect.height / 2;
    const radius = maxRadius(cx, cy);

    const overlay = document.createElement("div");
    overlay.setAttribute("aria-hidden", "true");
    Object.assign(overlay.style, {
      position: "fixed",
      inset: "0",
      zIndex: "9999",
      pointerEvents: "none",
      background: newBackgroundHex,
      clipPath: `circle(0px at ${cx}px ${cy}px)`,
    });

    document.body.appendChild(overlay);

    // 强制 reflow 后激活动画
    void overlay.offsetWidth;

    overlay.style.transition = `clip-path 500ms cubic-bezier(0.05, 0.7, 0.1, 1)`;
    overlay.style.clipPath = `circle(${radius}px at ${cx}px ${cy}px)`;

    function onEnd() {
      overlay.removeEventListener("transitionend", onEnd);
      overlay.remove();
      resolve();
    }

    overlay.addEventListener("transitionend", onEnd);

    // 兜底清理
    setTimeout(() => {
      if (overlay.parentNode) {
        overlay.remove();
        resolve();
      }
    }, 600);
  });
}
