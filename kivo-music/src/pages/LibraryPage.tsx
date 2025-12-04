// src/pages/LibraryPage.tsx
import React, { useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useLibrary } from "../store/library";
import { usePlayerStore } from "../store/player";
import type { MusicTrack } from "../types";
import { TrackList } from "../components/TrackList";
import { LibraryHeader } from "../components/library/LibraryHeader";
import { LibrarySortBar } from "../components/library/LibrarySortBar";

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
        const file = (t.filePath || (t as any).path || "").toLowerCase();
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
      const aValRaw =
        sortKey === "title" ? a.title || "" : a.artist || "";
      const bValRaw =
        sortKey === "title" ? b.title || "" : b.artist || "";
      const aVal = aValRaw.toLowerCase();
      const bVal = bValRaw.toLowerCase();

      if (aVal === bVal) return 0;
      const res = aVal < bVal ? -1 : 1;
      return sortAsc ? res : -res;
    });

    return sorted;
  }, [tracks, keyword, sortKey, sortAsc]);

  const total = tracks?.length ?? 0;
  const filteredCount = displayedTracks.length;

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
      (t as any).id ??
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

  // 顶部按钮：播放全部 / 随机播放
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

  const hasDisplayedTracks = displayedTracks.length > 0;

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
        gap: 12,
      }}
    >
      <LibraryHeader
        total={total}
        filteredCount={filteredCount}
        keyword={keyword}
        onKeywordChange={setKeyword}
        hasDisplayedTracks={hasDisplayedTracks}
        onPlayAll={handlePlayAll}
        onShufflePlay={handleShufflePlay}
        onImport={handleImport}
        onClearLibrary={clearLibrary}
      />

      <LibrarySortBar
        sortKey={sortKey}
        sortAsc={sortAsc}
        onToggleSort={toggleSort}
      />

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
