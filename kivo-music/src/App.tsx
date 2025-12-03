import React from "react";
import { AudioEngine } from "./components/AudioEngine";
import { TrackList } from "./components/TrackList";
import { PlayerBar } from "./components/PlayerBar";

const tabButtonStyle = (active: boolean): React.CSSProperties => ({
  padding: "4px 10px",
  fontSize: 13,
  borderRadius: 4,
  border: "1px solid " + (active ? "#60a5fa" : "#e5e7eb"),
  background: active ? "#eff6ff" : "#ffffff",
});

const App: React.FC = () => {
  // 目前只有「资料库」页面，其他两个先做占位
  return (
    <div
      style={{
        fontFamily:
          'system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
        height: "100vh",
        display: "flex",
        flexDirection: "column",
      }}
    >
      {/* 隐藏的真实播放引擎 */}
      <AudioEngine />

      {/* 顶部标题栏 */}
      <header
        style={{
          padding: "8px 16px",
          borderBottom: "1px solid #e5e7eb",
          display: "flex",
          alignItems: "center",
          gap: 12,
        }}
      >
        <div style={{ fontSize: 22, fontWeight: 600 }}>🎵 Kivo Music</div>
        <div style={{ color: "#6b7280", fontSize: 12 }}>
          本地音乐播放器 · 本来 20+ 年 计划版
        </div>
      </header>

      {/* 顶部 tab */}
      <div
        style={{
          padding: "6px 16px",
          borderBottom: "1px solid #e5e7eb",
          display: "flex",
          gap: 8,
        }}
      >
        <button style={tabButtonStyle(true)}>资料库</button>
        <button style={tabButtonStyle(false)}>播放列表</button>
        <button style={tabButtonStyle(false)}>正在播放</button>
      </div>

      {/* 主体内容：本地音乐资料库 */}
      <main
        style={{
          flex: 1,
          padding: "10px 16px 8px",
          overflow: "auto",
        }}
      >
        <h2
          style={{
            fontSize: 20,
            fontWeight: 600,
            marginBottom: 4,
          }}
        >
          本地音乐资料库
        </h2>
        <p
          style={{
            fontSize: 13,
            color: "#4b5563",
            marginBottom: 10,
          }}
        >
          这里会显示你导入的本地歌曲列表，后面我们会把它升级为真正的资料库（带封面、搜索、排序等）。
        </p>

        <TrackList />
      </main>

      {/* 底部播放器 */}
      <footer
        style={{
          padding: "8px 16px",
          borderTop: "1px solid #e5e7eb",
        }}
      >
        <PlayerBar />
      </footer>
    </div>
  );
};

export default App;
