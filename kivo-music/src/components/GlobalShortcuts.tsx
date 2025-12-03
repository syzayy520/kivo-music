// src/components/GlobalShortcuts.tsx
import React, { useEffect } from "react";
import * as PlayerStore from "../store/player";

// 为了兼容不同版本的导出，这里做一层小适配：
// - 优先使用 usePlayerStore
// - 退而求其次使用 usePlayer
// 这样不用去改动你现有的 store/player.ts 文件。
const usePlayerSelector: any =
  (PlayerStore as any).usePlayerStore ??
  (PlayerStore as any).usePlayer ??
  null;

const noop = () => {};

/**
 * 全局键盘快捷键：
 * - 空格：播放 / 暂停
 * - 左方向键：上一首
 * - 右方向键：下一首
 * - Ctrl+F / Cmd+F：聚焦到资料库搜索框
 */
export const GlobalShortcuts: React.FC = () => {
  // 如果没拿到 hook，就直接什么都不做（避免在编译期报错）
  if (!usePlayerSelector) {
    console.warn(
      "[GlobalShortcuts] no usePlayerStore / usePlayer exported from store/player",
    );
    return null;
  }

  const togglePlay =
    usePlayerSelector((s: any) => s.togglePlay ?? noop);
  const next = usePlayerSelector((s: any) => s.next ?? noop);
  const prev = usePlayerSelector((s: any) => s.prev ?? noop);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const tag = target?.tagName;
      const editable = target?.isContentEditable;

      // 在输入框 / 文本区域 / 可编辑区域中就不要拦截（避免影响打字）
      if (
        tag === "INPUT" ||
        tag === "TEXTAREA" ||
        editable ||
        (target && (target as any).closest?.("[contenteditable=true]"))
      ) {
        return;
      }

      // 空格：播放 / 暂停
      if (e.code === "Space") {
        e.preventDefault();
        togglePlay();
        return;
      }

      // 左右方向键：上一首 / 下一首
      if (e.code === "ArrowRight") {
        e.preventDefault();
        next();
        return;
      }
      if (e.code === "ArrowLeft") {
        e.preventDefault();
        prev();
        return;
      }

      // Ctrl+F / Cmd+F：聚焦到资料库搜索框
      if (
        (e.ctrlKey || e.metaKey) &&
        (e.key === "f" || e.key === "F")
      ) {
        const input =
          document.querySelector<HTMLInputElement>(
            'input[placeholder*="搜索标题"]',
          ) ||
          document.querySelector<HTMLInputElement>(
            'input[placeholder*="搜索"]',
          );

        if (input) {
          e.preventDefault();
          input.focus();
          input.select();
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [togglePlay, next, prev]);

  return null;
};
