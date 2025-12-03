// src/components/GlobalShortcuts.tsx
import React, { useEffect } from "react";
import { usePlayerStore } from "../store/player";

/**
 * 全局快捷键：
 * - Space / MediaPlayPause: 播放 / 暂停
 * - ArrowLeft / MediaTrackPrevious: 上一首
 * - ArrowRight / MediaTrackNext: 下一首
 * - Ctrl+F / Cmd+F: 聚焦资料库搜索框（id = "kivo-library-search"）
 */
const GlobalShortcuts: React.FC = () => {
  const togglePlay = usePlayerStore(
    (s: any) => s.togglePlay ?? (() => {}),
  );
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const tag = target?.tagName?.toLowerCase() ?? "";
      const isTypingElement =
        tag === "input" ||
        tag === "textarea" ||
        (target?.isContentEditable ?? false);

      // 播放 / 暂停：Space 或多媒体键
      if (e.code === "Space" || e.key === " " || e.code === "MediaPlayPause") {
        // 在输入框里敲空格就别打断人家了
        if (isTypingElement && e.code !== "MediaPlayPause") return;
        e.preventDefault();
        togglePlay();
        return;
      }

      // 上一首
      if (
        e.code === "ArrowLeft" ||
        e.code === "MediaTrackPrevious"
      ) {
        if (isTypingElement) return;
        e.preventDefault();
        prev();
        return;
      }

      // 下一首
      if (
        e.code === "ArrowRight" ||
        e.code === "MediaTrackNext"
      ) {
        if (isTypingElement) return;
        e.preventDefault();
        next();
        return;
      }

      // Ctrl+F / Cmd+F：聚焦资料库搜索框
      if (
        (e.ctrlKey || e.metaKey) &&
        (e.key === "f" || e.key === "F")
      ) {
        const input = document.getElementById(
          "kivo-library-search",
        ) as HTMLInputElement | null;
        if (input) {
          e.preventDefault();
          input.focus();
          input.select();
        }
      }
    };

    window.addEventListener("keydown", handler);
    return () => {
      window.removeEventListener("keydown", handler);
    };
  }, [togglePlay, next, prev]);

  // 只负责挂监听器，本身不渲染任何 UI
  return null;
};

export { GlobalShortcuts };
export default GlobalShortcuts;
