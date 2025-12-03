// src/App.tsx
import React, { useEffect, useState } from "react";
import { AudioEngine } from "./components/AudioEngine";
import { PlayerBar } from "./components/PlayerBar";
import LibraryPage from "./pages/LibraryPage";
import PlaylistPage from "./pages/PlaylistPage";
import NowPlayingPage from "./pages/NowPlayingPage";
import SettingsPage from "./pages/SettingsPage";
import { usePlayerStore } from "./store/player";

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

  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const next = usePlayerStore((s) => s.next);
  const prev = usePlayerStore((s) => s.prev);

  // 全局键盘快捷键：Space / ← / →
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
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [togglePlay, next, prev]);

  return (
    <div
      style={{
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        background: "#f3f4f6",
        color: "#111827",
        fontFamily:
          '-apple-system, BlinkMacSystemFont, "SF Pro Text", system-ui, sans-serif',
      }}
    >
      {/* 全局音频引擎 */}
      <AudioEngine />

      {/* 顶部 Tab + 标题 */}
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
            本地播放器 · v2
          </span>
        </div>

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
            {tab === "library" && <LibraryPage />}
            {tab === "playlist" && <PlaylistPage />}
            {tab === "nowPlaying" && <NowPlayingPage />}
            {tab === "settings" && <SettingsPage />}
          </div>
        </div>
      </main>

      {/* 底部播放器条 */}
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
