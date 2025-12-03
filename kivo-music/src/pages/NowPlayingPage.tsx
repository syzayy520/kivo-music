// src/pages/NowPlayingPage.tsx
import React, { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";
import { setCoverForTrack, getCachedCoverPath } from "../persistence/CoverCache";

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const currentTime = usePlayerStore((s: any) => s.currentTime ?? 0);
  const duration = usePlayerStore((s: any) => s.duration ?? 0);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? false);
  const togglePlay = usePlayerStore((s: any) => s.togglePlay ?? (() => {}));
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));
  const seek = usePlayerStore((s: any) => s.seek ?? (() => {}));
  const setPlaylist = usePlayerStore((s: any) => s.setPlaylist ?? (() => {}));

  const track = playlist[currentIndex] ?? null;

  const [seeking, setSeeking] = useState(false);
  const [seekValue, setSeekValue] = useState(0);

  // 真正用于渲染封面的路径
  const [resolvedCoverPath, setResolvedCoverPath] = useState<string | null>(null);
  const [coverError, setCoverError] = useState(false);

  // 当前曲目变化时，尝试自动补齐封面：
  // 1. track.coverPath 已经有 → 直接用
  // 2. 否则从 CoverCache 索引里查一遍（covers.json）
  useEffect(() => {
    let cancelled = false;
    setCoverError(false);

    (async () => {
      if (!track) {
        setResolvedCoverPath(null);
        return;
      }

      // 优先用 track 自己的 coverPath
      if (track.coverPath) {
        setResolvedCoverPath(track.coverPath);
        return;
      }

      try {
        const cached = await getCachedCoverPath(track);
        if (!cached || cancelled) return;

        setResolvedCoverPath(cached);

        // 顺便把 playlist + 库里的 coverPath 补上，避免以后再丢
        const updated = playlist.map((t: any, idx: number) =>
          idx === currentIndex ? { ...t, coverPath: cached } : t,
        );
        setPlaylist(updated);
        try {
          await saveLibrary(updated as any[]);
        } catch (err) {
          console.error(
            "[NowPlayingPage] saveLibrary after getCachedCoverPath error:",
            err,
          );
        }
      } catch (err) {
        console.error("[NowPlayingPage] getCachedCoverPath error:", err);
      }
    })();

    return () => {
      cancelled = true;
    };
  }, [track && (track.id ?? track.filePath), currentIndex, playlist, setPlaylist]);

  const coverSrc = useMemo(() => {
    if (!resolvedCoverPath || coverError) return null;
    try {
      return convertFileSrc(String(resolvedCoverPath));
    } catch (err) {
      console.error("[NowPlayingPage] convertFileSrc error:", err);
      return null;
    }
  }, [resolvedCoverPath, coverError]);

  const handleSeekChange = (value: number) => {
    setSeeking(true);
    setSeekValue(value);
  };

  const handleSeekCommit = (value: number) => {
    setSeeking(false);
    seek(value);
  };

  const handlePickCover = async () => {
    if (!track) return;

    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Images", extensions: ["jpg", "jpeg", "png", "webp"] }],
    });

    if (!selected || Array.isArray(selected)) return;

    const imagePath = String(selected);

    try {
      const cachedPath = await setCoverForTrack(track, imagePath);
      if (!cachedPath) return;

      const updated = playlist.map((t: any, idx: number) =>
        idx === currentIndex ? { ...t, coverPath: cachedPath } : t,
      );
      setPlaylist(updated);

      try {
        await saveLibrary(updated as any[]);
      } catch (err) {
        console.error(
          "[NowPlayingPage] saveLibrary after setCoverForTrack error:",
          err,
        );
      }

      setResolvedCoverPath(cachedPath);
      setCoverError(false);
    } catch (err) {
      console.error("[NowPlayingPage] handlePickCover error:", err);
    }
  };

  const title = track?.title || "暂无正在播放";
  const artist = track?.artist || "未知艺人";
  const album = track?.album || "";

  const currentTimeDisplay = formatTime(currentTime);
  const durationDisplay = formatTime(duration);

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        padding: "16px 24px",
        boxSizing: "border-box",
      }}
    >
      <div
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
          gap: 24,
        }}
      >
        {/* 左侧封面卡片 */}
        <div
          style={{
            width: 340,
            minWidth: 280,
            maxWidth: 360,
            borderRadius: 20,
            padding: 16,
            background: "linear-gradient(145deg,#020617,#111827)",
            boxShadow:
              "0 18px 45px rgba(15,23,42,0.55), 0 0 0 1px rgba(148,163,184,0.15)",
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
          }}
        >
          <div
            style={{
              borderRadius: 16,
              padding: 12,
              background: "radial-gradient(circle at 0% 0%,#0ea5e9,#1d4ed8)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              marginBottom: 12,
              aspectRatio: "1 / 1",
              overflow: "hidden",
              position: "relative",
            }}
          >
            {coverSrc && !coverError ? (
              <img
                src={coverSrc}
                alt={title}
                style={{
                  width: "100%",
                  height: "100%",
                  objectFit: "cover",
                  borderRadius: 14,
                  display: "block",
                }}
                onError={() => setCoverError(true)}
              />
            ) : (
              <div
                style={{
                  width: "100%",
                  height: "100%",
                  borderRadius: 14,
                  background:
                    "radial-gradient(circle at 20% 0%,#0ea5e9,#111827 60%)",
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "center",
                  color: "#e5e7eb",
                  fontSize: 40,
                  fontWeight: 700,
                  letterSpacing: 6,
                }}
              >
                {title ? title.charAt(0) : "♪"}
              </div>
            )}
          </div>

          <div style={{ marginTop: 4 }}>
            <div
              style={{
                fontSize: 15,
                fontWeight: 600,
                color: "#e5e7eb",
                marginBottom: 4,
                lineHeight: 1.4,
              }}
            >
              {title}
            </div>
            <div
              style={{
                fontSize: 12,
                color: "#9ca3af",
                marginBottom: 2,
              }}
            >
              {artist}
            </div>
            {album && (
              <div
                style={{
                  fontSize: 11,
                  color: "#6b7280",
                }}
              >
                {album}
              </div>
            )}
          </div>

          <div
            style={{
              marginTop: 14,
              paddingTop: 12,
              borderTop: "1px solid rgba(148,163,184,0.25)",
            }}
          >
            <button
              onClick={handlePickCover}
              style={{
                width: "100%",
                borderRadius: 9999,
                padding: "6px 10px",
                fontSize: 13,
                border: "1px solid rgba(148,163,184,0.7)",
                background:
                  "radial-gradient(circle at 0% 0%,rgba(56,189,248,0.1),rgba(15,23,42,0.9))",
                color: "#e5e7eb",
                cursor: "pointer",
              }}
            >
              选择封面图片…
            </button>
            <p
              style={{
                marginTop: 6,
                fontSize: 11,
                color: "#9ca3af",
                lineHeight: 1.5,
              }}
            >
              选择本地图片作为当前曲目的封面，应用会把图片复制到封面缓存目录中保存。
            </p>
          </div>
        </div>

        {/* 右侧信息区 */}
        <div
          style={{
            flex: 1,
            minWidth: 0,
            paddingTop: 4,
            display: "flex",
            flexDirection: "column",
          }}
        >
          <div
            style={{
              marginBottom: 16,
            }}
          >
            <div
              style={{
                fontSize: 16,
                fontWeight: 600,
                marginBottom: 4,
              }}
            >
              正在播放
            </div>
            {track ? (
              <div
                style={{
                  fontSize: 13,
                  color: "#6b7280",
                }}
              >
                来自当前播放列表：第 {currentIndex + 1} / {playlist.length} 首
              </div>
            ) : (
              <div
                style={{
                  fontSize: 13,
                  color: "#9ca3af",
                }}
              >
                当前没有正在播放的曲目。
              </div>
            )}
          </div>

          {/* 时间 + 播放控制（页面内的简化版） */}
          <div
            style={{
              marginBottom: 24,
            }}
          >
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                fontSize: 12,
                color: "#6b7280",
                marginBottom: 4,
              }}
            >
              <span>{currentTimeDisplay}</span>
              <span>{durationDisplay}</span>
            </div>
            <input
              type="range"
              min={0}
              max={duration || 0}
              step={0.1}
              value={seeking ? seekValue : currentTime}
              onChange={(e) => handleSeekChange(Number(e.target.value) || 0)}
              onMouseUp={(e) =>
                handleSeekCommit(Number((e.target as HTMLInputElement).value) || 0)
              }
              onTouchEnd={(e) =>
                handleSeekCommit(Number((e.target as HTMLInputElement).value) || 0)
              }
              style={{
                width: "100%",
                cursor: "pointer",
              }}
            />
            <div
              style={{
                marginTop: 12,
                display: "flex",
                gap: 8,
              }}
            >
              <button
                onClick={prev}
                style={{
                  padding: "4px 10px",
                  fontSize: 13,
                  borderRadius: 9999,
                  border: "1px solid #e5e7eb",
                  background: "#ffffff",
                  cursor: "pointer",
                }}
              >
                上一首
              </button>
              <button
                onClick={togglePlay}
                style={{
                  padding: "4px 16px",
                  fontSize: 13,
                  borderRadius: 9999,
                  border: "1px solid #2563eb",
                  background: isPlaying ? "#2563eb" : "#ffffff",
                  color: isPlaying ? "#ffffff" : "#2563eb",
                  cursor: "pointer",
                }}
              >
                {isPlaying ? "暂停" : "播放"}
              </button>
              <button
                onClick={next}
                style={{
                  padding: "4px 10px",
                  fontSize: 13,
                  borderRadius: 9999,
                  border: "1px solid #e5e7eb",
                  background: "#ffffff",
                  cursor: "pointer",
                }}
              >
                下一首
              </button>
            </div>
          </div>

          {/* 右侧其余空白，未来可以放歌词 / 队列预览等 */}
          <div
            style={{
              flex: 1,
              minHeight: 0,
              borderRadius: 16,
              border: "1px dashed #e5e7eb",
              background: "#ffffff",
              padding: 16,
              fontSize: 13,
              color: "#9ca3af",
            }}
          >
            封面和时间信息会与底部播放器联动同步。
            以后可以在这里扩展歌词、接下来播放的曲目等。
          </div>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
