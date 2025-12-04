import React, { useEffect, useState } from "react";

type LogLevel = "debug" | "info" | "warn" | "error";

const LOG_LEVEL_KEY = "kivo.developer.logLevel";
const SHOW_DEBUG_OVERLAY_KEY = "kivo.developer.showDebugOverlay";
const VISUALIZER_ENABLED_KEY = "kivo.feature.visualizer.enabled";
const VISUALIZER_TOGGLE_EVENT = "kivo-visualizer-toggle";

export const DeveloperSettingsPanel: React.FC = () => {
  const [logLevel, setLogLevel] = useState<LogLevel>("info");
  const [showDebugOverlay, setShowDebugOverlay] = useState(false);
  const [visualizerEnabled, setVisualizerEnabled] = useState(false);

  // 初次挂载时，从 localStorage 读取旧值
  useEffect(() => {
    try {
      const storedLevel = window.localStorage.getItem(LOG_LEVEL_KEY) as
        | LogLevel
        | null;
      if (storedLevel) setLogLevel(storedLevel);

      const storedOverlay = window.localStorage.getItem(
        SHOW_DEBUG_OVERLAY_KEY,
      );
      if (storedOverlay != null) {
        setShowDebugOverlay(storedOverlay === "1");
      }

      const storedVisualizer = window.localStorage.getItem(
        VISUALIZER_ENABLED_KEY,
      );
      if (storedVisualizer != null) {
        setVisualizerEnabled(storedVisualizer === "1");
      }
    } catch {
      // ignore
    }
  }, []);

  // 日志等级（预留）
  useEffect(() => {
    try {
      window.localStorage.setItem(LOG_LEVEL_KEY, logLevel);
      // TODO: 将来接入 log.ts: log.setLevel(logLevel)
    } catch {
      // ignore
    }
  }, [logLevel]);

  // 调试 Overlay（预留）
  useEffect(() => {
    try {
      window.localStorage.setItem(
        SHOW_DEBUG_OVERLAY_KEY,
        showDebugOverlay ? "1" : "0",
      );
      // TODO: 将来接入全局调试 Overlay
    } catch {
      // ignore
    }
  }, [showDebugOverlay]);

  // 频谱可视化开关
  useEffect(() => {
    try {
      window.localStorage.setItem(
        VISUALIZER_ENABLED_KEY,
        visualizerEnabled ? "1" : "0",
      );
    } catch {
      // ignore
    }

    try {
      const ev = new CustomEvent(VISUALIZER_TOGGLE_EVENT, {
        detail: { enabled: visualizerEnabled },
      });
      window.dispatchEvent(ev);
    } catch {
      // ignore
    }
  }, [visualizerEnabled]);

  return (
    <div
      style={{
        padding: 16,
        display: "flex",
        flexDirection: "column",
        gap: 16,
      }}
    >
      <h2 style={{ margin: 0, fontSize: 18 }}>开发者设置（预留）</h2>
      <p style={{ margin: "4px 0 8px", color: "#888", fontSize: 13 }}>
        这些选项当前只会保存在本机 localStorage 中，暂时不会直接影响播放内核。
        以后可以在此基础上接入 log.ts、调试信息 Overlay、频谱可视化等功能。
      </p>

      {/* 日志等级 */}
      <div>
        <label style={{ fontSize: 13, fontWeight: 500 }}>日志等级（预留）</label>
        <div style={{ marginTop: 4 }}>
          <select
            value={logLevel}
            onChange={(e) => setLogLevel(e.target.value as LogLevel)}
            style={{
              fontSize: 13,
              padding: "4px 8px",
              borderRadius: 4,
              border: "1px solid rgba(255,255,255,0.18)",
              background: "transparent",
              color: "inherit",
            }}
          >
            <option value="debug">Debug（最详细）</option>
            <option value="info">Info（默认）</option>
            <option value="warn">Warn（仅警告+错误）</option>
            <option value="error">Error（仅错误）</option>
          </select>
        </div>
        <p style={{ margin: "4px 0 0", color: "#888", fontSize: 12 }}>
          将来可以连接 log.ts 控制全局日志输出。
        </p>
      </div>

      {/* 调试 Overlay 开关 */}
      <div>
        <label style={{ fontSize: 13, fontWeight: 500 }}>
          <input
            type="checkbox"
            checked={showDebugOverlay}
            onChange={(e) => setShowDebugOverlay(e.target.checked)}
            style={{ marginRight: 6 }}
          />
          显示调试信息 Overlay（预留）
        </label>
        <p style={{ margin: "4px 0 0", color: "#888", fontSize: 12 }}>
          将来可以在界面上显示当前播放状态、内存占用等实时调试信息。
        </p>
      </div>

      {/* 频谱可视化开关 */}
      <div>
        <label style={{ fontSize: 13, fontWeight: 500 }}>
          <input
            type="checkbox"
            checked={visualizerEnabled}
            onChange={(e) => setVisualizerEnabled(e.target.checked)}
            style={{ marginRight: 6 }}
          />
          启用 Web Audio 频谱可视化（实验性）
        </label>
        <p style={{ margin: "4px 0 0", color: "#888", fontSize: 12 }}>
          开启后，“正在播放”页面会显示一个简单的频谱条形图，
          可能会略微增加 CPU 占用。
        </p>
      </div>
    </div>
  );
};
