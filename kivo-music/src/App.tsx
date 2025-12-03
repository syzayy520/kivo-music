// src/App.tsx
import React, { useState } from "react";
import { AudioEngine } from "./components/AudioEngine";
import { PlayerBar } from "./components/PlayerBar";
import PlaylistPage from "./pages/PlaylistPage";
import NowPlayingPage from "./pages/NowPlayingPage";
import LibraryPage from "./pages/LibraryPage";
import SettingsPage from "./pages/SettingsPage";
import { GlobalShortcuts } from "./components/GlobalShortcuts";

type TabKey = "library" | "playlist" | "nowPlaying" | "settings";

const tabButtonStyle = (
  active: boolean,
): React.CSSProperties => ({
  padding: "4px 10px",
  fontSize: 13,
  borderRadius: 4,
  border: "1px solid " + (active ? "#60a5fa" : "#e5e7eb"),
  background: active ? "#eff6ff" : "#ffffff",
  color: active ? "#1d4ed8" : "#4b5563",
  cursor: "pointer",
});

const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabKey>("library");

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
        fontFamily:
          "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
        background: "#f3f4f6",
      }}
    >
      {/* 隐藏的音频引擎（真正驱动 <audio> 播放） */}
      <AudioEngine />

      {/* 全局快捷键：空格播放/暂停、左右切歌、Ctrl+F 聚焦搜索 */}
      <GlobalShortcuts />

      <header
        style={{
          padding: "8px 16px",
          borderBottom: "1px solid #e5e7eb",
          background: "#ffffff",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "baseline",
            gap: 8,
          }}
        >
          <div
            style={{
              fontSize: 18,
              fontWeight: 600,
              color: "#111827",
            }}
          >
            Kivo Music
          </div>
          <div
            style={{
              fontSize: 11,
              color: "#9ca3af",
            }}
          >
            本地音乐播放器 · 未来 20+ 年计划版
          </div>
        </div>

        <nav
          style={{
            display: "flex",
            gap: 8,
          }}
        >
          <button
            type="button"
            style={tabButtonStyle(activeTab === "library")}
            onClick={() => setActiveTab("library")}
          >
            资料库
          </button>
          <button
            type="button"
            style={tabButtonStyle(activeTab === "playlist")}
            onClick={() => setActiveTab("playlist")}
          >
            播放列表
          </button>
          <button
            type="button"
            style={tabButtonStyle(activeTab === "nowPlaying")}
            onClick={() => setActiveTab("nowPlaying")}
          >
            正在播放
          </button>
          <button
            type="button"
            style={tabButtonStyle(activeTab === "settings")}
            onClick={() => setActiveTab("settings")}
          >
            设置
          </button>
        </nav>
      </header>

      <main
        style={{
          flex: 1,
          padding: "12px 16px 0",
          overflow: "hidden",
        }}
      >
        <div
          style={{
            height: "100%",
            borderRadius: 12,
            background: "#ffffff",
            boxShadow: "0 10px 30px rgba(15,23,42,0.08)",
            padding: "12px 16px",
            boxSizing: "border-box",
            overflow: "hidden",
          }}
        >
          {activeTab === "library" && <LibraryPage />}
          {activeTab === "playlist" && <PlaylistPage />}
          {activeTab === "nowPlaying" && <NowPlayingPage />}
          {activeTab === "settings" && <SettingsPage />}
        </div>
      </main>

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
