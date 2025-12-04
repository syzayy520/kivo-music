// src/App.tsx
import React, { useCallback, useEffect, useState } from "react";
import { AudioEngine } from "./components/AudioEngine";
import { PlayerBar } from "./components/PlayerBar";
import LibraryPage from "./pages/LibraryPage";
import PlaylistPage from "./pages/PlaylistPage";
import NowPlayingPage from "./pages/NowPlayingPage";
import SettingsPage from "./pages/SettingsPage";
import { usePlayerStore } from "./store/player";

// Tauri v2 窗口 API
import { getCurrentWindow } from "@tauri-apps/api/window";
import { PhysicalSize } from "@tauri-apps/api/dpi";

// Mini 模式组件（下面我也给你实现，如果你还没有的话）
import MiniPlayer from "./components/MiniPlayer";

type TabKey = "library" | "playlist" | "nowPlaying" | "settings";

const tabButtonStyle = (active: boolean): React.CSSProperties => ({
  padding: "4px 10px",
  fontSize: 13,
  borderRadius: 9999,
  border: "1px solid " + (active ? "#2563eb" : "transparent"),
  background: active ? "#eff6ff" : "transparent",
  color: active ? "#1d4ed8" : "#4b5563",
  cursor: "pointer",
});

const App: React.FC = () => {
  const [tab, setTab] = useState<TabKey>("library");
  const [miniMode, setMiniMode] = useState(false);

  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const next = usePlayerStore((s) => s.next);
  const prev = usePlayerStore((s) => s.prev);

  // 进入 Mini 模式：缩成小窗 + 置顶 + 禁止拉伸
  const enterMiniMode = useCallback(async () => {
    setMiniMode(true);
    try {
      const win = getCurrentWindow();
      await win.setAlwaysOnTop(true);
      await win.setResizable(false);
      await win.setSize(new PhysicalSize(420, 560));
    } catch (error) {
      console.error("[App] 进入 Mini 模式时调整窗口失败:", error);
    }
  }, []);

  // 退出 Mini 模式：恢复正常窗口大小
  const exitMiniMode = useCallback(async () => {
    setMiniMode(false);
    try {
      const win = getCurrentWindow();
      await win.setAlwaysOnTop(false);
      await win.setResizable(true);
      await win.setSize(new PhysicalSize(1200, 800));
    } catch (error) {
      console.error("[App] 退出 Mini 模式时调整窗口失败:", error);
    }
  }, []);

  // 全局键盘快捷键：
  // Space：播放/暂停
  // ← / →：上一首 / 下一首
  // M：切换 Mini 模式
  // Esc：强制退出 Mini 模式
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;

      // 在输入框里就不抢快捷键
      if (
        target &&
        (target.tagName === "INPUT" ||
          target.tagName === "TEXTAREA" ||
          target.isContentEditable)
      ) {
        return;
      }

      if (e.code === "Space") {
        e.preventDefault();
        togglePlay();
      } else if (e.code === "ArrowRight") {
        next();
      } else if (e.code === "ArrowLeft") {
        prev();
      } else if (e.code === "KeyM") {
        e.preventDefault();
        if (miniMode) {
          void exitMiniMode();
        } else {
          void enterMiniMode();
        }
      } else if (e.code === "Escape" && miniMode) {
        e.preventDefault();
        void exitMiniMode();
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [togglePlay, next, prev, miniMode, enterMiniMode, exitMiniMode]);

  const renderMainPage = () => {
    if (tab === "library") return <LibraryPage />;
    if (tab === "playlist") return <PlaylistPage />;
    if (tab === "nowPlaying") return <NowPlayingPage />;
    if (tab === "settings") return <SettingsPage />;
    return null;
  };

  const rootStyle: React.CSSProperties = {
    height: "100vh",
    display: "flex",
    flexDirection: "column",
    background: "#f3f4f6",
    color: "#111827",
    fontFamily:
      '-apple-system, BlinkMacSystemFont, "SF Pro Text", system-ui, sans-serif',
  };

  // Mini 模式：整个窗口只展示 MiniPlayer，小窗尺寸由上面的 enter/exitMiniMode 控制
  if (miniMode) {
    return (
      <div style={rootStyle}>
        <AudioEngine />
        <MiniPlayer onExitMiniMode={() => void exitMiniMode()} />
      </div>
    );
  }

  // 普通模式（带顶部 Tab + 底部 PlayerBar）
  return (
    <div style={rootStyle}>
      {/* 全局音频引擎 */}
      <AudioEngine />

      {/* 顶部 Tab + 标题 + Mini 按钮 */}
      <header
        style={{
          padding: "8px 16px",
          borderBottom: "1px solid #e5e7eb",
          background: "#ffffff",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 16,
        }}
      >
        {/* 左侧 Logo / 标题 */}
        <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
          <div
            style={{
              width: 8,
              height: 8,
              borderRadius: "9999px",
              background:
                "radial-gradient(circle at 30% 30%, #22c55e, #16a34a)",
              marginRight: 4,
            }}
          />
          <span style={{ fontWeight: 600, fontSize: 14 }}>Kivo Music</span>
          <span
            style={{
              fontSize: 11,
              color: "#9ca3af",
              padding: "1px 6px",
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              marginLeft: 4,
            }}
          >
            本地播放器 · v3
          </span>
        </div>

        {/* 右侧 Tab + Mini 模式按钮 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
          }}
        >
          <nav
            style={{
              display: "flex",
              alignItems: "center",
              gap: 6,
              background: "#f3f4f6",
              borderRadius: 9999,
              padding: 2,
            }}
          >
            <button
              style={tabButtonStyle(tab === "library")}
              onClick={() => setTab("library")}
            >
              资料库
            </button>
            <button
              style={tabButtonStyle(tab === "playlist")}
              onClick={() => setTab("playlist")}
            >
              播放列表
            </button>
            <button
              style={tabButtonStyle(tab === "nowPlaying")}
              onClick={() => setTab("nowPlaying")}
            >
              正在播放
            </button>
            <button
              style={tabButtonStyle(tab === "settings")}
              onClick={() => setTab("settings")}
            >
              设置
            </button>
          </nav>

          <button
            style={{
              padding: "4px 10px",
              fontSize: 12,
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              background: "#111827",
              color: "#f9fafb",
              cursor: "pointer",
              whiteSpace: "nowrap",
            }}
            onClick={() => {
              void enterMiniMode();
            }}
          >
            极简 Mini 模式
          </button>
        </div>
      </header>

      {/* 中间内容区 */}
      <main
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            flex: 1,
            minHeight: 0,
            overflow: "hidden",
          }}
        >
          <div
            style={{
              height: "100%",
              overflow: "auto",
            }}
          >
            {renderMainPage()}
          </div>
        </div>
      </main>

      {/* 底部播放器条（Mini 模式下会隐藏，这里是普通模式所以照常显示） */}
      <footer
        style={{
          padding: "8px 16px",
          borderTop: "1px solid #e5e7eb",
          background: "#ffffff",
        }}
      >
        <PlayerBar />
      </footer>
    </div>
  );
};

export default App;
