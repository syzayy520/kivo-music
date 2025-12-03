// src/App.tsx
import React, { useState } from "react";
import { AudioEngine } from "./components/AudioEngine";
import { TrackList } from "./components/TrackList";
import { PlayerBar } from "./components/PlayerBar";
import PlaylistPage from "./pages/PlaylistPage";
import NowPlayingPage from "./pages/NowPlayingPage";

type TabKey = "library" | "playlist" | "nowPlaying";

const tabButtonStyle = (active: boolean): React.CSSProperties => ({
  padding: "4px 10px",
  fontSize: 13,
  borderRadius: 4,
  border: "1px solid " + (active ? "#60a5fa" : "#e5e7eb"),
  background: active ? "#eff6ff" : "#ffffff",
  color: active ? "#1d4ed8" : "#374151",
  cursor: "pointer",
});

/**
 * 顶层 App：
 * - 上面是标题 + Tab 切换（资料库 / 播放列表 / 正在播放）
 * - 中间根据当前 Tab 显示对应页面
 * - 底部是全局 PlayerBar
 * - AudioEngine 在最顶层挂一次就好
 */
const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabKey>("library");

  let content: React.ReactNode;
  if (activeTab === "library") {
    content = <TrackList />;
  } else if (activeTab === "playlist") {
    content = <PlaylistPage />;
  } else {
    content = <NowPlayingPage />;
  }

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
        fontFamily:
          '-apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif',
        background: "#f9fafb",
      }}
    >
      {/* 音频引擎，全局挂载一次 */}
      <AudioEngine />

      {/* 顶部标题栏 + Tab */}
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
        <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
          <span style={{ fontSize: 20 }}>🎵</span>
          <div>
            <div
              style={{
                fontSize: 16,
                fontWeight: 600,
                lineHeight: 1.2,
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
              本地音乐播放器 · 本来 20+ 年计划版
            </div>
          </div>
        </div>

        <nav style={{ display: "flex", gap: 8 }}>
          <button
            style={tabButtonStyle(activeTab === "library")}
            onClick={() => setActiveTab("library")}
          >
            资料库
          </button>
          <button
            style={tabButtonStyle(activeTab === "playlist")}
            onClick={() => setActiveTab("playlist")}
          >
            播放列表
          </button>
          <button
            style={tabButtonStyle(activeTab === "nowPlaying")}
            onClick={() => setActiveTab("nowPlaying")}
          >
            正在播放
          </button>
        </nav>
      </header>

      {/* 中间主内容区域 */}
      <main
        style={{
          flex: 1,
          overflow: "auto",
          background: "#f9fafb",
        }}
      >
        {content}
      </main>

      {/* 底部播放器 */}
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
