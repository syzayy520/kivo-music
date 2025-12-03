// src/pages/NowPlayingPage.tsx
import React, { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

function getTrackDisplayTitle(track: any | null): string {
  if (!track) return "暂无播放";
  if (track.title && String(track.title).trim().length > 0) {
    return String(track.title);
  }
  const filePath = track.filePath ? String(track.filePath) : "";
  const parts = filePath.split(/[/\\]/);
  const name = parts[parts.length - 1] || "";
  return name || "未知曲目";
}

function getTrackDisplayArtist(track: any | null): string {
  if (!track) return "";
  if (track.artist && String(track.artist).trim().length > 0) {
    return String(track.artist);
  }
  return "未知艺人";
}

function getInitialsFromTitle(title: string): string {
  const t = title.trim();
  if (!t) return "♪";
  // 中文歌名直接拿前两个字；其他语言拿第一个字符
  if (/[\u4e00-\u9fa5]/.test(t[0]) && t.length >= 2) {
    return t.slice(0, 2);
  }
  return t[0].toUpperCase();
}

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const currentTime = usePlayerStore((s: any) => s.currentTime ?? 0);
  const duration = usePlayerStore((s: any) => s.duration ?? 0);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? false);
  const togglePlay = usePlayerStore(
    (s: any) => s.togglePlay ?? (() => {}),
  );
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  ) as (tracks: any[]) => void;

  const hasTrack =
    Array.isArray(playlist) &&
    playlist.length > 0 &&
    currentIndex >= 0 &&
    currentIndex < playlist.length;

  const track = hasTrack ? playlist[currentIndex] : null;

  // 当前封面是否加载失败（例如封面文件被删除）
  const [coverError, setCoverError] = useState(false);

  // 每次切歌时重置封面错误标记
  useEffect(() => {
    setCoverError(false);
  }, [track && (track.id ?? track.filePath)]);

  const coverSrc = useMemo(() => {
    if (!track || coverError) return null;

    // 只使用记录在库里的 coverPath；如果文件被删除，这里会返回 null，
    // 封面卡片会显示默认渐变背景。
    if (track.coverPath) {
      try {
        return convertFileSrc(String(track.coverPath));
      } catch {
        return null;
      }
    }

    return null;
  }, [track, coverError]);

  const title = getTrackDisplayTitle(track);
  const artist = getTrackDisplayArtist(track);
  const album =
    track && track.album && String(track.album).trim().length > 0
      ? String(track.album)
      : "";

  const displayInitials = getInitialsFromTitle(title);

  const handlePickCover = async () => {
    if (!track || !hasTrack) return;

    try {
      const result = await open({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["jpg", "jpeg", "png", "webp", "bmp"],
          },
        ],
      });

      if (!result) return;

      const path = Array.isArray(result) ? result[0] : result;
      const fullPath = String(path);

      const updated = playlist.map((t: any, idx: number) =>
        idx === currentIndex ? { ...t, coverPath: fullPath } : t,
      );

      setPlaylist(updated);

      try {
        await saveLibrary(updated as any[]);
        console.info("[NowPlaying] saveLibrary with cover ok");
      } catch (error) {
        console.error("[NowPlaying] saveLibrary failed:", error);
      }
    } catch (error) {
      console.error("[NowPlaying] pick cover failed:", error);
    }
  };

  const indexText = hasTrack
    ? `来自当前播放列表：第 ${currentIndex + 1} / ${playlist.length} 首`
    : "当前没有正在播放的歌曲";

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        gap: 24,
      }}
    >
      {/* 左侧大封面卡片 */}
      <div
        style={{
          flex: "0 0 320px",
          display: "flex",
          flexDirection: "column",
          borderRadius: 16,
          padding: 16,
          background:
            "radial-gradient(circle at top left, #1d4ed8 0, #020617 45%, #020617 100%)",
          color: "#e5e7eb",
          boxShadow: "0 18px 45px rgba(15,23,42,0.55)",
        }}
      >
        <div
          style={{
            flex: 1,
            borderRadius: 12,
            background: "linear-gradient(145deg, #020617, #0f172a)",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
            marginBottom: 16,
          }}
        >
          {coverSrc && !coverError ? (
            <img
              src={coverSrc}
              style={{
                width: "100%",
                height: "100%",
                objectFit: "cover",
              }}
              onError={() => setCoverError(true)}
            />
          ) : (
            <div
              style={{
                fontSize: 40,
                fontWeight: 700,
                letterSpacing: 4,
              }}
            >
              {displayInitials}
            </div>
          )}
        </div>

        <div>
          <div
            style={{
              fontSize: 16,
              fontWeight: 600,
              marginBottom: 4,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {title}
          </div>
          <div
            style={{
              fontSize: 13,
              color: "#9ca3af",
              marginBottom: 12,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {artist}
            {album ? ` · ${album}` : ""}
          </div>

          <button
            onClick={handlePickCover}
            style={{
              padding: "6px 10px",
              borderRadius: 8,
              border: "1px solid #e5e7eb",
              background: "#f9fafb",
              color: "#111827",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            🖼 选择封面图片…
          </button>
        </div>
      </div>

      {/* 右侧信息 + 控制区 */}
      <div
        style={{
          flex: 1,
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <div>
          <div
            style={{
              fontSize: 14,
              color: "#6b7280",
              marginBottom: 4,
            }}
          >
            正在播放
          </div>
          <div
            style={{
              fontSize: 20,
              fontWeight: 600,
              marginBottom: 4,
            }}
          >
            {title}
          </div>
          <div
            style={{
              fontSize: 14,
              color: "#6b7280",
              marginBottom: 12,
            }}
          >
            {indexText}
          </div>

          <div
            style={{
              display: "flex",
              alignItems: "baseline",
              gap: 8,
              fontSize: 13,
              color: "#6b7280",
              marginBottom: 12,
            }}
          >
            <span>{formatTime(currentTime)}</span>
            <span>/</span>
            <span>{formatTime(duration)}</span>
          </div>

          <p
            style={{
              fontSize: 12,
              color: "#9ca3af",
              maxWidth: 520,
              lineHeight: 1.5,
            }}
          >
            封面和曲目信息会与底部播放器保持同步。你可以在资料库或播放列表中切歌，
            这里会自动跟随更新；也可以使用顶部的 Tab 在各个页面间自由切换。
          </p>
        </div>

        <div
          style={{
            display: "flex",
            gap: 12,
            marginTop: 24,
            flexWrap: "wrap",
          }}
        >
          <button
            onClick={prev}
            style={{
              padding: "6px 10px",
              borderRadius: 6,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            ⏮ 上一首
          </button>
          <button
            onClick={togglePlay}
            style={{
              padding: "6px 10px",
              borderRadius: 6,
              border: "1px solid #2563eb",
              background: "#2563eb",
              color: "#ffffff",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            {isPlaying ? "⏸ 暂停" : "▶ 播放"}
          </button>
          <button
            onClick={next}
            style={{
              padding: "6px 10px",
              borderRadius: 6,
              border: "1px solid #e5e7eb",
              background: "#ffffff",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            ⏭ 下一首
          </button>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
