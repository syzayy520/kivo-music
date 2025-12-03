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

  // 从播放器 store 中取出我们需要的两个操作
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  );
  const playTrackByIndex = usePlayerStore(
    (s: any) => s.playTrack ?? (() => {}),
  );
    const playerPlaylist = usePlayerStore(
    (s: any) => s.playlist ?? s.tracks ?? [],
  );
  const currentIndex = usePlayerStore(
    (s: any) => s.currentIndex ?? -1,
  );
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


  const [keyword, setKeyword] = useState("");
  const [sortKey, setSortKey] = useState<SortKey>("none");
  const [sortAsc, setSortAsc] = useState(true);

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

      // 同步到播放器队列：用最新的全库作为 playlist
      const allTracks = useLibrary.getState().tracks;
      if (allTracks && allTracks.length > 0) {
        setPlaylist(allTracks);
      }
    } catch (e) {
      console.error("导入音乐文件失败:", e);
    }
  };

  // 播放指定歌曲：根据 id 在全库中找下标
  const handlePlayTrack = (track: MusicTrack, _index: number) => {
    const allTracks = useLibrary.getState().tracks;
    if (!allTracks || allTracks.length === 0) return;

    const index = allTracks.findIndex((t) => t.id === track.id);
    if (index === -1) return;

    setPlaylist(allTracks);
    playTrackByIndex(index);
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

  // 计算需要展示的列表：先过滤、再排序
  const displayedTracks = useMemo(() => {
    let list = tracks || [];

    const kw = keyword.trim().toLowerCase();
    if (kw) {
      list = list.filter((t: MusicTrack) => {
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

    if (sortKey === "none") {
      return list;
    }

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
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            placeholder="搜索标题 / 艺人 / 专辑"
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
