// src/components/now-playing/VisualizerPanel.tsx
import React, { useEffect, useRef, useState } from "react";
import { getFrequencyData } from "../../audio-backend/VisualizerBus";
import { useI18n } from "../../i18n";

const VISUALIZER_ENABLED_KEY = "kivo.feature.visualizer.enabled";
const VISUALIZER_TOGGLE_EVENT = "kivo-visualizer-toggle";

export const VisualizerPanel: React.FC = () => {
  const { t } = useI18n();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [enabled, setEnabled] = useState(false);

  // 读取“是否启用频谱可视化”的设置，并监听切换事件
  useEffect(() => {
    let initial = false;
    try {
      initial = window.localStorage.getItem(VISUALIZER_ENABLED_KEY) === "1";
    } catch {
      // ignore storage errors
    }
    setEnabled(initial);

    const handler = (event: Event) => {
      const detail = (event as CustomEvent<{ enabled?: boolean }>).detail;
      if (detail && typeof detail.enabled === "boolean") {
        setEnabled(detail.enabled);
      }
    };

    window.addEventListener(VISUALIZER_TOGGLE_EVENT, handler as EventListener);
    return () => {
      window.removeEventListener(
        VISUALIZER_TOGGLE_EVENT,
        handler as EventListener,
      );
    };
  }, []);

  // 频谱绘制循环（当前为“伪频谱”，不影响真实音频路径）
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let animationFrameId: number;

    const disabledText = t("nowPlaying.visualizer.canvas.disabled");
    const idleText = t("nowPlaying.visualizer.canvas.idle");

    const render = () => {
      const width = canvas.width;
      const height = canvas.height;

      ctx.clearRect(0, 0, width, height);

      if (!enabled) {
        ctx.fillStyle = "rgba(15,23,42,0.85)";
        ctx.fillRect(0, 0, width, height);
        ctx.font =
          "12px system-ui, -apple-system, BlinkMacSystemFont, sans-serif";
        ctx.fillStyle = "rgba(148,163,184,0.85)";
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText(disabledText, width / 2, height / 2);
      } else {
        const data = getFrequencyData();
        if (data && data.length > 0) {
          const barCount = Math.min(64, data.length);
          const step = Math.floor(data.length / barCount) || 1;
          const barWidth = width / barCount;

          for (let i = 0; i < barCount; i++) {
            const v = data[i * step] ?? 0;
            const ratio = v / 255;
            const barHeight = ratio * height;
            const x = i * barWidth;
            const y = height - barHeight;

            const gradient = ctx.createLinearGradient(x, y, x, height);
            gradient.addColorStop(0, "rgba(96,165,250,0.95)");
            gradient.addColorStop(1, "rgba(15,23,42,0.1)");

            ctx.fillStyle = gradient;
            ctx.fillRect(
              x + barWidth * 0.18,
              y,
              barWidth * 0.64,
              barHeight,
            );
          }
        } else {
          ctx.fillStyle = "rgba(15,23,42,0.85)";
          ctx.fillRect(0, 0, width, height);
          ctx.font =
            "12px system-ui, -apple-system, BlinkMacSystemFont, sans-serif";
          ctx.fillStyle = "rgba(148,163,184,0.85)";
          ctx.textAlign = "center";
          ctx.textBaseline = "middle";
          ctx.fillText(idleText, width / 2, height / 2);
        }
      }

      animationFrameId = window.requestAnimationFrame(render);
    };

    animationFrameId = window.requestAnimationFrame(render);

    return () => {
      window.cancelAnimationFrame(animationFrameId);
    };
  }, [enabled, t]);

  return (
    <div
      style={{
        padding: 12,
        borderRadius: 10,
        border: "1px solid rgba(148,163,184,0.25)",
        background:
          "linear-gradient(135deg, rgba(15,23,42,0.96), rgba(15,23,42,0.99))",
      }}
    >
      <div
        style={{
          marginBottom: 8,
          fontSize: 12,
          opacity: 0.8,
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          color: "#e5e7eb",
        }}
      >
        <span>{t("nowPlaying.visualizer.title")}</span>
        <span style={{ fontSize: 11, opacity: 0.7 }}>
          {enabled
            ? t("nowPlaying.visualizer.status.enabled")
            : t("nowPlaying.visualizer.status.disabled")}
        </span>
      </div>
      <canvas
        ref={canvasRef}
        width={600}
        height={120}
        style={{
          width: "100%",
          height: 120,
          display: "block",
        }}
      />
    </div>
  );
};
