// src/components/MiniPlayer.tsx
import React from "react";
import { usePlayerStore } from "../store/player";
import { useNowPlayingCover } from "../hooks/useNowPlayingCover";

interface MiniPlayerProps {
  onExitMiniMode?: () => void;
}

const MiniPlayer: React.FC<MiniPlayerProps> = ({ onExitMiniMode }) => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);
  const currentTime = usePlayerStore((s) => s.currentTime);
  const duration = usePlayerStore((s) => s.duration);
  const volume = usePlayerStore((s) => s.volume);

  const togglePlay = usePlayerStore((s) => s.togglePlay);
  const next = usePlayerStore((s) => s.next);
  const prev = usePlayerStore((s) => s.prev);
  const setVolume = usePlayerStore((s) => s.setVolume);

  const track = playlist[currentIndex] ?? null;
  const { coverSrc } = useNowPlayingCover(track, playlist, currentIndex);

  const safeProgress =
    duration && duration > 0 ? Math.min(1, Math.max(0, currentTime / duration)) : 0;

  const anyTrack = track as any;
  const title =
    anyTrack?.title ?? anyTrack?.name ?? anyTrack?.fileName ?? "未选择曲目";
  const artist =
    anyTrack?.artist ?? anyTrack?.albumArtist ?? "未知艺人";
  const album = anyTrack?.album ?? "";

  return (
    <div
      style={{
        flex: 1,
        display: "flex",
        flexDirection: "column",
        padding: 16,
        gap: 12,
      }}
    >
      {/* 顶部标题栏 */}
      <header
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          marginBottom: 8,
        }}
      >
        <div>
          <div style={{ fontSize: 14, fontWeight: 600 }}>Kivo Music · Mini</div>
          <div style={{ fontSize: 11, color: "#9ca3af" }}>
            {track ? "当前曲目" : "暂无曲目"}
          </div>
        </div>
        {onExitMiniMode && (
          <button
            onClick={onExitMiniMode}
            style={{
              padding: "4px 10px",
              fontSize: 12,
              borderRadius: 9999,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              color: "#111827",
              cursor: "pointer",
              whiteSpace: "nowrap",
            }}
          >
            返回完整模式
          </button>
        )}
      </header>

      {/* 封面 + 文本信息 */}
      <div
        style={{
          flex: 1,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 12,
        }}
      >
        <div
          style={{
            width: 240,
            height: 240,
            borderRadius: 24,
            overflow: "hidden",
            background: "#e5e7eb",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
          }}
        >
          {coverSrc ? (
            <img
              src={coverSrc}
              alt="cover"
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
                display: "block",
              }}
            />
          ) : (
            <span style={{ fontSize: 12, color: "#6b7280" }}>暂无封面</span>
          )}
        </div>

        <div style={{ textAlign: "center" }}>
          <div
            style={{
              fontSize: 14,
              fontWeight: 600,
              marginBottom: 2,
              maxWidth: 260,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {title}
          </div>
          <div
            style={{
              fontSize: 12,
              color: "#6b7280",
              maxWidth: 260,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {artist}
            {album ? ` · ${album}` : ""}
          </div>
        </div>
      </div>

      {/* 进度条 + 控制区 + 音量 */}
      <div
        style={{
          marginTop: "auto",
          display: "flex",
          flexDirection: "column",
          gap: 8,
        }}
      >
        {/* 进度条（只读） */}
        <div
          style={{
            width: "100%",
            height: 4,
            borderRadius: 9999,
            background: "#e5e7eb",
            overflow: "hidden",
          }}
        >
          <div
            style={{
              width: `${safeProgress * 100}%`,
              height: "100%",
              background: "#3b82f6",
              transition: "width 0.1s linear",
            }}
          />
        </div>

        {/* 播放控制按钮 */}
        <div
          style={{
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
            gap: 16,
            marginTop: 4,
          }}
        >
          <button
            onClick={prev}
            style={{
              width: 36,
              height: 36,
              borderRadius: "9999px",
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
              fontSize: 14,
            }}
          >
            ◀
          </button>
          <button
            onClick={togglePlay}
            style={{
              width: 44,
              height: 44,
              borderRadius: "9999px",
              border: "none",
              background: "#111827",
              color: "#f9fafb",
              cursor: "pointer",
              fontSize: 16,
            }}
          >
            {isPlaying ? "⏸" : "▶"}
          </button>
          <button
            onClick={next}
            style={{
              width: 36,
              height: 36,
              borderRadius: "9999px",
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              cursor: "pointer",
              fontSize: 14,
            }}
          >
            ▶
          </button>
        </div>

        {/* 音量条：0~1，和底部 PlayerBar 共享同一个 store */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
            marginTop: 6,
          }}
        >
          <span style={{ fontSize: 11, color: "#6b7280" }}>音量</span>
          <input
            type="range"
            min={0}
            max={1}
            step={0.01}
            value={volume}
            onChange={(e) => {
              const v = parseFloat(e.target.value);
              if (!Number.isNaN(v)) setVolume(v);
            }}
            style={{ flex: 1 }}
          />
        </div>
      </div>
    </div>
  );
};

export default MiniPlayer;
