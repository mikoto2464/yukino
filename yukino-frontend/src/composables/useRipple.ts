/*
 * ============================================================
 * Android MD3 Ripple 引擎 — 触摸涟漪反馈
 * 复刻 RippleDrawable 行为: 在 pointerdown 处创建圆形
 * 使用 MD3 emphasized easing 展开, accelerate easing 消退
 * 状态叠加层 (StateLayer) 由 CSS :hover/:active 处理
 * ============================================================
 */

const cleanupFns = new WeakMap<HTMLElement, () => void>();

/** 在宿主元素上注册 ripple 监听。调用方负责在 onUnmounted 中移除 */
export function useRipple(host: HTMLElement | null): void {
  if (!host) return;
  if (cleanupFns.has(host)) return; // 已注册

  const el = host; // capture non-null for closure safety

  // 确保宿主可以定位涟漪
  const prevPosition = el.style.position;
  if (!prevPosition || prevPosition === "static") {
    el.style.position = "relative";
  }
  el.style.overflow = "hidden";

  let activeCircle: HTMLSpanElement | null = null;
  let exitTimer: ReturnType<typeof setTimeout> | null = null;

  function removeCircle(circle: HTMLSpanElement) {
    circle.classList.add("md-ripple-circle--exiting");
    const onEnd = () => {
      circle.removeEventListener("animationend", onEnd);
      if (circle.parentNode) circle.remove();
      if (activeCircle === circle) activeCircle = null;
    };
    circle.addEventListener("animationend", onEnd);
    // 兜底清理
    setTimeout(() => {
      if (circle.parentNode) {
        circle.remove();
        if (activeCircle === circle) activeCircle = null;
      }
    }, 300);
  }

  function clearActive() {
    if (activeCircle) {
      removeCircle(activeCircle);
      activeCircle = null;
    }
  }

  function onPointerDown(e: PointerEvent) {
    // 忽略非主键
    if (e.button !== 0) return;
    // 忽略禁用态
    if (
      el.hasAttribute("disabled") ||
      el.getAttribute("aria-disabled") === "true"
    )
      return;

    clearActive();
    if (exitTimer) {
      clearTimeout(exitTimer);
      exitTimer = null;
    }

    const rect = el.getBoundingClientRect();
    // 涟漪覆盖长边 1.5 倍，确保圆形展开后完全覆盖
    const dim = Math.max(rect.width, rect.height);
    const size = dim * 1.5;
    const x = e.clientX - rect.left - size / 2;
    const y = e.clientY - rect.top - size / 2;

    const circle = document.createElement("span");
    circle.className = "md-ripple-circle";
    Object.assign(circle.style, {
      width: `${size}px`,
      height: `${size}px`,
      left: `${x}px`,
      top: `${y}px`,
    });

    el.appendChild(circle);

    // 强制 reflow 后激活动画
    void circle.offsetWidth;
    circle.classList.add("md-ripple-circle--entering");
    activeCircle = circle;
  }

  function onPointerUp(_e: PointerEvent) {
    if (activeCircle) {
      // 延迟消退，让用户看到满圆
      exitTimer = setTimeout(() => {
        clearActive();
        exitTimer = null;
      }, 100);
    }
  }

  function onPointerLeave(_e: PointerEvent) {
    if (activeCircle) {
      removeCircle(activeCircle);
      activeCircle = null;
      if (exitTimer) {
        clearTimeout(exitTimer);
        exitTimer = null;
      }
    }
  }

  function cleanup() {
    el.removeEventListener("pointerdown", onPointerDown);
    document.removeEventListener("pointerup", onPointerUp);
    el.removeEventListener("pointerleave", onPointerLeave);
    clearActive();
    if (exitTimer) clearTimeout(exitTimer);
  }

  el.addEventListener("pointerdown", onPointerDown);
  // pointerup 监听在 document 上以捕获在元素外的释放
  document.addEventListener("pointerup", onPointerUp);
  el.addEventListener("pointerleave", onPointerLeave);

  cleanupFns.set(el, cleanup);
}

/** 移除 ripple 监听，由调用方在 onUnmounted 中调用 */
export function removeRipple(host: HTMLElement | null): void {
  if (!host) return;
  const fn = cleanupFns.get(host);
  if (fn) {
    fn();
    cleanupFns.delete(host);
  }
}
