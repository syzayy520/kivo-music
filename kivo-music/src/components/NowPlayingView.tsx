import React from "react";
import { usePlayerStore } from "../store/player";
import type { MusicTrack } from "../types";

const formatTime = (value: number | undefined) => {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
};

export const NowPlayingView: React.FC = () => {
  // 为了兼容之前可能的字段名差异，这里把 state 当成 any 来做一层映射
  const state = usePlayerStore() as any;

  const queue: MusicTrack[] =
    state.tracks ?? state.playlist ?? state.queue ?? [];
  const currentIndex: number = state.currentIndex ?? 0;
  const isPlaying: boolean = !!(state.isPlaying ?? state.playing);
  const time: number =
    state.position ?? state.currentTime ?? state.time ?? 0;
  const duration: number = state.duration ?? 0;
  const mode: string = state.mode ?? "normal";

  const next: () => void = state.next ?? (() => {});
  const prev: () => void = state.prev ?? state.previous ?? (() => {});
  const togglePlay: () => void =
    state.togglePlay ?? state.setPlaying ?? (() => {});

  const current = queue[currentIndex] as MusicTrack | undefined;

  if (!current) {
    return (
      <div style={{ padding: "16px 0" }}>
        <h2 style={{ fontSize: 20, fontWeight: 600, marginBottom: 8 }}>
          正在播放
        </h2>
        <p>当前没有正在播放的歌曲，可以先去「资料库」点一首歌试试。</p>
      </div>
    );
  }

  const { title, artist, album } = current;

  return (
    <div style={{ padding: "16px 0" }}>
      <h2 style={{ fontSize: 20, fontWeight: 600, marginBottom: 16 }}>
        正在播放
      </h2>

      <div
        style={{
          display: "flex",
          gap: 24,
          alignItems: "center",
          flexWrap: "wrap",
        }}
      >
        {/* 封面占位（后面可以换成真正封面） */}
        <div
          style={{
            width: 200,
            height: 200,
            borderRadius: 16,
            background:
              "linear-gradient(135deg, #f5f5f5 0%, #e5e7eb 40%, #d1d5db 100%)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            fontSize: 32,
            fontWeight: 700,
          }}
        >
          {title?.charAt(0) || "♪"}
        </div>

        {/* 歌曲信息 */}
        <div style={{ minWidth: 260, flex: 1 }}>
          <div style={{ marginBottom: 12 }}>
            <div style={{ fontSize: 24, fontWeight: 600, marginBottom: 4 }}>
              {title || "未知标题"}
            </div>
            <div style={{ color: "#4b5563", marginBottom: 2 }}>
              {artist || "未知艺人"}
            </div>
            {album && (
              <div style={{ color: "#6b7280", fontSize: 14 }}>
                专辑：{album}
              </div>
            )}
          </div>

          {/* 播放状态 & 时间信息（只读展示，真正控制交给底部 PlayerBar） */}
          <div style={{ fontSize: 14, color: "#4b5563", marginBottom: 8 }}>
            <div>
              播放状态：{isPlaying ? "正在播放" : "已暂停"} · 模式：{mode}
            </div>
            <div>
              进度：{formatTime(time)} / {formatTime(duration)}
            </div>
          </div>

          {/* 简单的上一首 / 播放暂停 / 下一首按钮 */}
          <div style={{ display: "flex", gap: 8, marginTop: 8 }}>
            <button onClick={prev}>⏮ 上一首</button>
            <button onClick={togglePlay}>
              {isPlaying ? "⏸ 暂停" : "▶ 播放"}
            </button>
            <button onClick={next}>⏭ 下一首</button>
          </div>

          <div style={{ marginTop: 8, fontSize: 12, color: "#9ca3af" }}>
            提示：时间进度、音量等细节仍然由底部的控制条（PlayerBar）负责，
            这里主要是更直观地展示当前播放的歌曲信息。
          </div>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingView;
