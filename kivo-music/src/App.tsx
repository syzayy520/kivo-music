// src/App.tsx
import React, { useCallback, useEffect, useState } from "react";
import { AudioEngine } from "./components/AudioEngine";
import { usePlayerStore } from "./store/player";
import MiniPlayer from "./components/MiniPlayer";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { PhysicalSize } from "@tauri-apps/api/dpi";

import { MainLayout } from "./components/layout/MainLayout";
import type { TabKey } from "./navigation/navigationModel";
import { kivoTheme } from "./styles/theme";

const rootStyle: React.CSSProperties = {
  height: "100vh",
  display: "flex",
  flexDirection: "column",
  background: kivoTheme.colors.appBackground,
  color: kivoTheme.colors.textOnDark,
  fontFamily:
    '-apple-system, BlinkMacSystemFont, "SF Pro Text", system-ui, sans-serif',
};

const App: React.FC = () => {
  const [tab, setTab] = useState<TabKey>("library");
  const [miniMode, setMiniMode] = useState(false);

  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const next = usePlayerStore((s) => s.next);
  const prev = usePlayerStore((s) => s.prev);

  // 进入 Mini 模式
  const enterMiniMode = useCallback(async () => {
    setMiniMode(true);

    const win = getCurrentWindow();

    // 1) 先尝试置顶，如果权限不允许，只打 warn，不中断后续逻辑
    try {
      await win.setAlwaysOnTop(true);
    } catch (error) {
      console.warn(
        "[App] 进入 Mini 模式时尝试置顶窗口失败（通常是权限未开启），已忽略：",
        error,
      );
    }

    // 2) 再单独控制可缩放与窗口尺寸
    try {
      await win.setResizable(false);
    } catch (error) {
      console.warn("[App] 进入 Mini 模式时设置窗口为不可缩放失败：", error);
    }

    try {
      // 这里先写一个相对适中的 Mini 大小，后续可以再微调
      await win.setSize(new PhysicalSize(420, 220));
    } catch (error) {
      console.warn("[App] 进入 Mini 模式时调整窗口尺寸失败：", error);
    }
  }, []);

  // 退出 Mini 模式，恢复正常窗口
  const exitMiniMode = useCallback(async () => {
    setMiniMode(false);

    const win = getCurrentWindow();

    try {
      await win.setAlwaysOnTop(false);
    } catch (error) {
      console.warn("[App] 退出 Mini 模式时取消置顶失败：", error);
    }

    try {
      await win.setResizable(true);
    } catch (error) {
      console.warn("[App] 退出 Mini 模式时恢复窗口可缩放失败：", error);
    }

    try {
      await win.setSize(new PhysicalSize(1200, 800));
    } catch (error) {
      console.warn("[App] 退出 Mini 模式时恢复窗口尺寸失败：", error);
    }
  }, []);

  // 全局键盘快捷键
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      // 空格：播放/暂停（不在输入框时）
      if (e.code === "Space") {
        const active = document.activeElement;
        if (
          active &&
          (active.tagName === "INPUT" ||
            active.tagName === "TEXTAREA" ||
            (active as HTMLElement).isContentEditable)
        ) {
          return;
        }
        e.preventDefault();
        togglePlay();
      }

      // Ctrl + 右：下一首
      if (e.ctrlKey && e.code === "ArrowRight") {
        e.preventDefault();
        next();
      }

      // Ctrl + 左：上一首
      if (e.ctrlKey && e.code === "ArrowLeft") {
        e.preventDefault();
        prev();
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [togglePlay, next, prev]);

  return (
    <div style={rootStyle}>
      <AudioEngine />
      {miniMode ? (
        <MiniPlayer onExitMiniMode={exitMiniMode} />
      ) : (
        <MainLayout
          currentTab={tab}
          onChangeTab={setTab}
          onEnterMiniMode={enterMiniMode}
        />
      )}
    </div>
  );
};

export default App;
