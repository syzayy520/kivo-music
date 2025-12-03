// src/pages/LibraryPage.tsx
import React, { useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useLibrary } from "../store/library";
import { usePlayerStore } from "../store/player";
import type { MusicTrack } from "../types";
import { TrackList } from "../components/TrackList";

function pathToTitle(path: string): string {
  const parts = path.split(/[/\\]/);
  const file = parts[parts.length - 1] || path;
  const dotIndex = file.lastIndexOf(".");
  return dotIndex > 0 ? file.slice(0, dotIndex) : file;
}

type SortKey = "none" | "title" | "artist";

const LibraryPage: React.FC = () => {
  const { tracks, addTracks, clearLibrary } = useLibrary();

  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  ) as (tracks: MusicTrack[]) => void;
  const playTrackByIndex = usePlayerStore(
    (s: any) => s.playTrack ?? (() => {}),
  ) as (index: number) => void;

  const playerPlaylist = usePlayerStore(
    (s: any) => s.playlist ?? s.tracks ?? [],
  );
  const currentIndex = usePlayerStore(
    (s: any) => s.currentIndex ?? -1,
  );

  const [keyword, setKeyword] = useState("");
  const [sortKey, setSortKey] = useState<SortKey>("none");
  const [sortAsc, setSortAsc] = useState(true);

  // ===== 导入本地音乐 =====
  const handleImport = async () => {
    try {
      const result = await open({
        multiple: true,
        filters: [
          {
            name: "Audio",
            extensions: ["mp3", "flac", "wav", "ogg", "m4a", "ape"],
          },
        ],
      });

      if (!result) return;

      const files = Array.isArray(result) ? result : [result];

      const newTracks: MusicTrack[] = files.map((filePath) => {
        const fullPath = String(filePath);
        const title = pathToTitle(fullPath);
        return {
          id: fullPath,
          filePath: fullPath,
          path: fullPath,
          title,
          artist: "未知艺人",
          album: "未知专辑",
          duration: 0,
        };
      });

      addTracks(newTracks);

      // 默认行为：导入后按“整个库”的顺序作为播放列表
      const allTracks = useLibrary.getState().tracks as MusicTrack[];
      if (allTracks && allTracks.length > 0) {
        setPlaylist(allTracks);
      }
    } catch (e) {
      console.error("导入音乐文件失败:", e);
    }
  };

  // ===== 排序 / 过滤后的“当前列表” =====
  const displayedTracks: MusicTrack[] = useMemo(() => {
    let list = (tracks ?? []) as MusicTrack[];

    const kw = keyword.trim().toLowerCase();
    if (kw) {
      list = list.filter((t) => {
        const title = (t.title || "").toLowerCase();
        const artist = (t.artist || "").toLowerCase();
        const album = (t.album || "").toLowerCase();
        const file = (t.filePath || t.path || "").toLowerCase();
        return (
          title.includes(kw) ||
          artist.includes(kw) ||
          album.includes(kw) ||
          file.includes(kw)
        );
      });
    }

    if (sortKey === "none") return list;

    const sorted = [...list].sort((a, b) => {
      const aVal =
        sortKey === "title"
          ? (a.title || "").toLowerCase()
          : (a.artist || "").toLowerCase();
      const bVal =
        sortKey === "title"
          ? (b.title || "").toLowerCase()
          : (b.artist || "").toLowerCase();
      if (aVal === bVal) return 0;
      const res = aVal < bVal ? -1 : 1;
      return sortAsc ? res : -res;
    });

    return sorted;
  }, [tracks, keyword, sortKey, sortAsc]);

  const total = tracks?.length ?? 0;
  const filteredCount = displayedTracks.length;

  const sortLabel = (key: SortKey) => {
    if (sortKey !== key) return "";
    return sortAsc ? "▲" : "▼";
  };

  const activeTrackId = useMemo(() => {
    if (
      currentIndex < 0 ||
      currentIndex >= playerPlaylist.length
    ) {
      return null;
    }
    const t = playerPlaylist[currentIndex];
    if (!t) return null;
    return (
      t.id ??
      (t as any).filePath ??
      (t as any).path ??
      null
    );
  }, [playerPlaylist, currentIndex]);

  // ===== 双击某一行播放：以“当前显示列表”的顺序作为队列 =====
  const handlePlayTrack = (track: MusicTrack, index: number) => {
    if (!displayedTracks.length) return;
    setPlaylist(displayedTracks);
    playTrackByIndex(index);
  };

  // ===== 顶部“播放全部 / 随机播放”按钮 =====
  const handlePlayAll = () => {
    if (!displayedTracks.length) return;
    setPlaylist(displayedTracks);
    playTrackByIndex(0);
  };

  const handleShufflePlay = () => {
    if (!displayedTracks.length) return;
    const randomIndex = Math.floor(
      Math.random() * displayedTracks.length,
    );
    setPlaylist(displayedTracks);
    playTrackByIndex(randomIndex);
  };

  // 切换排序
  const toggleSort = (key: SortKey) => {
    if (key === "none") {
      setSortKey("none");
      return;
    }
    if (sortKey === key) {
      setSortAsc((v) => !v);
    } else {
      setSortKey(key);
      setSortAsc(true);
    }
  };

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        gap: 12,
      }}
    >
      {/* 顶部标题 + 搜索 + 按钮 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: 12,
        }}
      >
        <div>
          <h1
            style={{
              fontSize: 18,
              fontWeight: 600,
              marginBottom: 4,
            }}
          >
            本地音乐资料库
          </h1>
          <p
            style={{
              fontSize: 12,
              color: "#6b7280",
            }}
          >
            共 {total} 首歌曲
            {keyword && ` · 匹配到 ${filteredCount} 首`}
          </p>
        </div>

        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 8,
          }}
        >
          <input
            id="kivo-library-search"
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            placeholder="搜索标题 / 艺人 / 专辑"
            autoComplete="off"
            spellCheck={false}
            style={{
              minWidth: 220,
              borderRadius: 6,
              border: "1px solid #d1d5db",
              padding: "4px 8px",
              fontSize: 13,
              outline: "none",
            }}
          />

          <button
            type="button"
            onClick={handlePlayAll}
            disabled={!displayedTracks.length}
            style={{
              padding: "4px 10px",
              borderRadius: 6,
              border: "1px solid #22c55e",
              backgroundColor: displayedTracks.length
                ? "#22c55e"
                : "#e5e7eb",
              color: displayedTracks.length ? "#ffffff" : "#9ca3af",
              fontSize: 12,
              cursor: displayedTracks.length ? "pointer" : "default",
            }}
          >
            ▶ 播放全部
          </button>

          <button
            type="button"
            onClick={handleShufflePlay}
            disabled={!displayedTracks.length}
            style={{
              padding: "4px 10px",
              borderRadius: 6,
              border: "1px solid #3b82f6",
              backgroundColor: displayedTracks.length
                ? "#ffffff"
                : "#e5e7eb",
              color: displayedTracks.length ? "#1d4ed8" : "#9ca3af",
              fontSize: 12,
              cursor: displayedTracks.length ? "pointer" : "default",
            }}
          >
            🔀 随机播放
          </button>

          <button
            type="button"
            onClick={handleImport}
            style={{
              padding: "4px 10px",
              borderRadius: 6,
              border: "none",
              background: "#8b5cf6",
              color: "#ffffff",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            + 导入本地音乐文件
          </button>

          <button
            type="button"
            onClick={clearLibrary}
            style={{
              padding: "4px 10px",
              borderRadius: 6,
              border: "1px solid #d1d5db",
              background: "#ffffff",
              color: "#4b5563",
              fontSize: 13,
              cursor: "pointer",
            }}
          >
            清空资料库
          </button>
        </div>
      </div>

      {/* 排序按钮 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          fontSize: 12,
          color: "#6b7280",
        }}
      >
        <div style={{ display: "flex", gap: 12 }}>
          <button
            type="button"
            onClick={() => toggleSort("none")}
            style={{
              border: "none",
              background: "transparent",
              cursor: "pointer",
              color: sortKey === "none" ? "#111827" : "#6b7280",
            }}
          >
            默认顺序
          </button>
          <button
            type="button"
            onClick={() => toggleSort("title")}
            style={{
              border: "none",
              background: "transparent",
              cursor: "pointer",
              color: sortKey === "title" ? "#111827" : "#6b7280",
            }}
          >
            标题 {sortLabel("title")}
          </button>
          <button
            type="button"
            onClick={() => toggleSort("artist")}
            style={{
              border: "none",
              background: "transparent",
              cursor: "pointer",
              color: sortKey === "artist" ? "#111827" : "#6b7280",
            }}
          >
            艺人 {sortLabel("artist")}
          </button>
        </div>
      </div>

      {/* 列表区域 */}
      <div style={{ flex: 1 }}>
        <TrackList
          tracks={displayedTracks}
          onPlay={handlePlayTrack}
          activeTrackId={activeTrackId}
        />
      </div>
    </div>
  );
};

export default LibraryPage;
