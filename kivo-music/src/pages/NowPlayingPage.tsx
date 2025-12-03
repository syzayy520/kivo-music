// src/pages/NowPlayingPage.tsx
//
// B2.2：在 B2.1 基础上，增加「目录封面索引持久化」
// - 自动封面（folder/cover.jpg）加载失败时，会把该目录记入 CoverCache 的 folders 列表
// - 下次启动时，先查询 isFolderKnownNoCover，命中后根本不会再去请求 cover.jpg，自然不会再刷 500

import React, {
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import { saveLibrary } from "../persistence/LibraryPersistence";
import {
  setCoverForTrack,
  isFolderKnownNoCover,
  markFolderNoCover,
} from "../persistence/CoverCache";

function formatTime(value: number | undefined): string {
  if (!value || !Number.isFinite(value)) return "0:00";
  const total = Math.floor(value);
  const m = Math.floor(total / 60);
  const s = total % 60;
  return `${m}:${s.toString().padStart(2, "0")}`;
}

// 内存级别的「这个目录没有封面」缓存：
// - 当前运行中用它避免重复调用 isFolderKnownNoCover
// - 同时配合持久化版本（CoverCache.folders）
const noCoverFolders = new Set<string>();

function getFolderPathFromTrack(track: any | null): string | null {
  if (!track) return null;
  const p: string | undefined = track.filePath ?? track.path;
  if (!p) return null;
  const str = String(p);
  const parts = str.split(/[/\\]/);
  if (parts.length <= 1) return null;
  parts.pop();
  const sep = str.includes("\\") ? "\\" : "/";
  return parts.join(sep);
}

// 简单规则：同一目录下的 cover.jpg 作为自动封面候选
function guessCoverPathForTrack(
  track: any | null,
): { folderPath: string | null; candidatePath: string | null } {
  if (!track) return { folderPath: null, candidatePath: null };
  const folderPath = getFolderPathFromTrack(track);
  if (!folderPath) return { folderPath: null, candidatePath: null };

  const sep = folderPath.includes("\\") ? "\\" : "/";
  const candidate = `${folderPath}${sep}cover.jpg`;
  return { folderPath, candidatePath: candidate };
}

function hashString(input: string): number {
  let hash = 0;
  for (let i = 0; i < input.length; i++) {
    hash = (hash * 31 + input.charCodeAt(i)) | 0;
  }
  return hash;
}

function pickGradientForKey(key: any): string {
  const str = String(key ?? "default");
  const h = Math.abs(hashString(str));
  const palettes = [
    "linear-gradient(135deg, #0f172a, #1e293b)",
    "linear-gradient(135deg, #111827, #4b5563)",
    "linear-gradient(135deg, #0f172a, #334155)",
    "linear-gradient(135deg, #020617, #1f2937)",
  ];
  return palettes[h % palettes.length];
}

const NowPlayingPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const isPlaying = usePlayerStore((s: any) => s.isPlaying ?? false);
  const currentTime = usePlayerStore((s: any) => s.currentTime ?? 0);
  const duration = usePlayerStore((s: any) => s.duration ?? 0);
  const togglePlay = usePlayerStore(
    (s: any) => s.togglePlay ?? (() => {}),
  );
  const next = usePlayerStore((s: any) => s.next ?? (() => {}));
  const prev = usePlayerStore((s: any) => s.prev ?? (() => {}));
  const seek = usePlayerStore((s: any) => s.seek ?? (() => {}));
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  );

  const track =
    currentIndex >= 0 && currentIndex < playlist.length
      ? playlist[currentIndex]
      : null;

  const [coverError, setCoverError] = useState(false);
  const [folderHasNoCover, setFolderHasNoCover] = useState(false);

  // 当前曲目所在目录
  const folderPath = useMemo(
    () => getFolderPathFromTrack(track),
    [track && (track as any).id, currentIndex],
  );

  // 用来标记当前 <img> 的 src 是不是「猜的 cover.jpg」
  const coverFromGuessedRef = useRef(false);

  // track / index 变化时重置错误状态
  useEffect(() => {
    setCoverError(false);
  }, [track && (track as any).id, currentIndex]);

  // 目录发生变化时，从持久化索引里查询「是否已知没有封面」
  useEffect(() => {
    let cancelled = false;

    const checkFolder = async () => {
      if (!folderPath) {
        setFolderHasNoCover(false);
        return;
      }

      // 先看内存缓存
      if (noCoverFolders.has(folderPath)) {
        setFolderHasNoCover(true);
        return;
      }

      try {
        const known = await isFolderKnownNoCover(folderPath);
        if (cancelled) return;
        if (known) {
          noCoverFolders.add(folderPath);
          setFolderHasNoCover(true);
        } else {
          setFolderHasNoCover(false);
        }
      } catch (error) {
        console.warn(
          "[NowPlayingPage] isFolderKnownNoCover failed:",
          error,
        );
        if (!cancelled) {
          setFolderHasNoCover(false);
        }
      }
    };

    checkFolder();

    return () => {
      cancelled = true;
    };
  }, [folderPath]);

  const coverSrc = useMemo(() => {
    coverFromGuessedRef.current = false;

    if (!track) return null;
    if (coverError) return null;

    // 1. 优先使用手动选择的封面（走 CoverCache）
    if (track.coverPath) {
      try {
        return convertFileSrc(String(track.coverPath));
      } catch (e) {
        console.warn("[NowPlayingPage] convertFileSrc coverPath error:", e);
        return null;
      }
    }

    // 2. 没有手动封面，并且该目录已经被标记为「无封面」，则不再尝试 cover.jpg
    if (folderPath && folderHasNoCover) {
      return null;
    }

    // 3. 尝试使用目录下的 cover.jpg
    const { candidatePath } = guessCoverPathForTrack(track);
    if (!candidatePath) return null;

    try {
      coverFromGuessedRef.current = true;
      return convertFileSrc(candidatePath);
    } catch (e) {
      console.warn(
        "[NowPlayingPage] convertFileSrc guessed cover error:",
        e,
      );
      return null;
    }
  }, [track, folderPath, folderHasNoCover, coverError]);

  const handlePickCover = async () => {
    if (!track) return;

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

      const updatedTrack = await setCoverForTrack(track, fullPath);

      const updatedPlaylist = playlist.map((t: any, idx: number) =>
        idx === currentIndex ? updatedTrack : t,
      );
      setPlaylist(updatedPlaylist);

      try {
        await saveLibrary(updatedPlaylist);
      } catch (error) {
        console.error(
          "[NowPlayingPage] saveLibrary after set cover failed:",
          error,
        );
      }

      setCoverError(false);
    } catch (error) {
      console.error("[NowPlayingPage] pick cover error:", error);
    }
  };

  const handleImageError = () => {
    setCoverError(true);

    // 只有在「尝试目录 cover.jpg」失败时，才标记目录无封面
    if (folderPath && coverFromGuessedRef.current) {
      noCoverFolders.add(folderPath);
      setFolderHasNoCover(true);
      markFolderNoCover(folderPath).catch((error) => {
        console.error(
          "[NowPlayingPage] markFolderNoCover failed:",
          error,
        );
      });
    }
  };

  if (!track) {
    return (
      <div
        style={{
          padding: "24px 32px",
          height: "100%",
          boxSizing: "border-box",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          color: "#6b7280",
          fontSize: 14,
        }}
      >
        当前没有正在播放的歌曲。
      </div>
    );
  }

  const title = track.title || track.name || "未知标题";
  const artist = track.artist || "未知艺人";
  const album = track.album || "";
  const gradient = pickGradientForKey(
    track.id ?? track.filePath ?? track.path,
  );

  return (
    <div
      style={{
        padding: "24px 32px",
        height: "100%",
        boxSizing: "border-box",
      }}
    >
      <div
        style={{
          display: "flex",
          gap: 24,
          alignItems: "stretch",
          height: "100%",
        }}
      >
        {/* 左侧：封面卡片 */}
        <div
          style={{
            flex: "0 0 320px",
            maxWidth: 360,
            borderRadius: 18,
            padding: 20,
            background: gradient,
            color: "#e5e7eb",
            boxShadow: "0 18px 40px rgba(15,23,42,0.6)",
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
          }}
        >
          <div>
            <div
              style={{
                width: "100%",
                paddingBottom: "100%",
                borderRadius: 14,
                overflow: "hidden",
                position: "relative",
                background:
                  "radial-gradient(circle at 0% 0%, #1f2937, #020617)",
                marginBottom: 16,
                border: "1px solid rgba(148,163,184,0.25)",
              }}
            >
              {coverSrc && !coverError ? (
                <img
                  src={coverSrc}
                  alt={title}
                  onError={handleImageError}
                  style={{
                    position: "absolute",
                    inset: 0,
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                  }}
                />
              ) : (
                <div
                  style={{
                    position: "absolute",
                    inset: 0,
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    fontSize: 32,
                    fontWeight: 600,
                    letterSpacing: 2,
                    textTransform: "uppercase",
                    color: "#9ca3af",
                  }}
                >
                  {String(title).slice(0, 2)}
                </div>
              )}
            </div>

            <div style={{ marginBottom: 12 }}>
              <div
                style={{
                  fontSize: 18,
                  fontWeight: 600,
                  color: "#f9fafb",
                  marginBottom: 4,
                }}
              >
                {title}
              </div>
              <div
                style={{
                  fontSize: 13,
                  color: "#d1d5db",
                }}
              >
                {artist}
                {album && ` · ${album}`}
              </div>
            </div>
          </div>

          <button
            type="button"
            onClick={handlePickCover}
            style={{
              marginTop: 8,
              width: "100%",
              padding: "6px 10px",
              borderRadius: 999,
              border: "1px solid rgba(148,163,184,0.8)",
              background: "rgba(15,23,42,0.6)",
              color: "#e5e7eb",
              fontSize: 13,
              cursor: "pointer",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              gap: 6,
            }}
          >
            <span style={{ fontSize: 14 }}>🖼️</span>
            <span>选择封面图片…</span>
          </button>
        </div>

        {/* 右侧：曲目信息 + 播放控制 */}
        <div
          style={{
            flex: 1,
            display: "flex",
            flexDirection: "column",
            justifyContent: "space-between",
          }}
        >
          <div>
            <h2
              style={{
                fontSize: 22,
                fontWeight: 600,
                marginBottom: 4,
              }}
            >
              正在播放
            </h2>
            <p
              style={{
                fontSize: 13,
                color: "#6b7280",
                marginBottom: 16,
              }}
            >
              来自当前播放列表 · 第 {currentIndex + 1} 首 / 共{" "}
              {playlist.length} 首
            </p>

            <div
              style={{
                padding: 14,
                borderRadius: 12,
                border: "1px solid #e5e7eb",
                background: "#f9fafb",
                marginBottom: 24,
              }}
            >
              <div
                style={{
                  fontSize: 16,
                  fontWeight: 600,
                  marginBottom: 4,
                }}
              >
                {title}
              </div>
              <div
                style={{
                  fontSize: 13,
                  color: "#6b7280",
                  marginBottom: 8,
                }}
              >
                {artist}
                {album && ` · ${album}`}
              </div>

              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 8,
                  fontSize: 12,
                  color: "#6b7280",
                }}
              >
                <span>{formatTime(currentTime)}</span>
                <input
                  type="range"
                  min={0}
                  max={duration || 0}
                  step={0.1}
                  value={
                    duration > 0 &&
                    currentTime >= 0 &&
                    currentTime <= duration
                      ? currentTime
                      : 0
                  }
                  onChange={(e) =>
                    seek ? seek(Number(e.target.value) || 0) : undefined
                  }
                  style={{
                    flex: 1,
                    cursor: "pointer",
                  }}
                />
                <span>{formatTime(duration)}</span>
              </div>
            </div>
          </div>

          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              gap: 24,
              marginBottom: 12,
            }}
          >
            <button
              type="button"
              onClick={() => prev && prev()}
              style={{
                width: 40,
                height: 40,
                borderRadius: 999,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                cursor: "pointer",
                fontSize: 16,
              }}
            >
              ⏮
            </button>
            <button
              type="button"
              onClick={() => togglePlay && togglePlay()}
              style={{
                width: 56,
                height: 56,
                borderRadius: 999,
                border: "none",
                background: "#111827",
                color: "#f9fafb",
                cursor: "pointer",
                fontSize: 20,
                fontWeight: 600,
                boxShadow: "0 12px 30px rgba(15,23,42,0.6)",
              }}
            >
              {isPlaying ? "⏸" : "▶️"}
            </button>
            <button
              type="button"
              onClick={() => next && next()}
              style={{
                width: 40,
                height: 40,
                borderRadius: 999,
                border: "1px solid #e5e7eb",
                background: "#ffffff",
                cursor: "pointer",
                fontSize: 16,
              }}
            >
              ⏭
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NowPlayingPage;
