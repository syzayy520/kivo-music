// 路径：src/pages/LibraryPage.tsx
import React, { useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useLibrary } from "../store/library";
import {
  LibraryTrack,
  SortKey,
  getTrackTitle,
  getTrackArtist,
  getTrackAlbum,
  startPlaylistFrom,
  saveCurrentLibrary,
} from "../library/libraryModel";
import { LibraryHeader } from "../components/library/LibraryHeader";
import { LibraryTracksView } from "../components/library/LibraryTracksView";
import { LibraryGroupViews } from "../components/library/LibraryGroupViews";
import { useI18n } from "../i18n";

type ViewMode = "tracks" | "albums" | "artists";

const LibraryPage: React.FC = () => {
  // 从库 store 里拿数据 & 方法
  const tracks: LibraryTrack[] = useLibrary((s: any) => s.tracks ?? []);
  const addTracks = useLibrary((s: any) => s.addTracks ?? null);
  const clearLibrary = useLibrary((s: any) => s.clearLibrary ?? null);

  const { t } = useI18n();

  // 视图状态：歌曲 / 专辑 / 艺人
  const [viewMode, setViewMode] = useState<ViewMode>("tracks");
  // 搜索关键字
  const [search, setSearch] = useState("");
  // 排序方式
  const [sortKey, setSortKey] = useState<SortKey>("default");

  // 先按搜索过滤
  const filteredTracks = useMemo(() => {
    const keyword = search.trim().toLowerCase();
    let result = Array.isArray(tracks) ? tracks.slice() : [];

    if (keyword) {
      result = result.filter((t) => {
        const title = getTrackTitle(t).toLowerCase();
        const artist = getTrackArtist(t).toLowerCase();
        const album = (getTrackAlbum(t) || "").toLowerCase();
        const pathText = (
          (t as any).filePath ||
          (t as any).path ||
          (t as any).location ||
          ""
        )
          .toString()
          .toLowerCase();

        return (
          title.includes(keyword) ||
          artist.includes(keyword) ||
          album.includes(keyword) ||
          pathText.includes(keyword)
        );
      });
    }

    return result;
  }, [tracks, search]);

  // 再按排序规则排一遍
  const tracksForView = useMemo(() => {
    const result = filteredTracks.slice();

    switch (sortKey) {
      case "title":
        result.sort((a, b) =>
          getTrackTitle(a).localeCompare(getTrackTitle(b), "zh-CN"),
        );
        break;
      case "artist":
        result.sort((a, b) =>
          getTrackArtist(a).localeCompare(getTrackArtist(b), "zh-CN"),
        );
        break;
      case "album":
        result.sort((a, b) => {
          const albumA = getTrackAlbum(a) || "";
          const albumB = getTrackAlbum(b) || "";
          return albumA.localeCompare(albumB, "zh-CN");
        });
        break;
      case "recent":
        result.sort((a, b) => {
          const aTime =
            (a as any).lastPlayedAt != null
              ? Date.parse((a as any).lastPlayedAt)
              : 0;
          const bTime =
            (b as any).lastPlayedAt != null
              ? Date.parse((b as any).lastPlayedAt)
              : 0;
          return bTime - aTime;
        });
        break;
      case "default":
      default:
        // 保持原顺序
        break;
    }

    return result;
  }, [filteredTracks, sortKey]);

  // 操作：播放全部
  const handlePlayAll = () => {
    if (!tracksForView.length) return;
    startPlaylistFrom(tracksForView, 0);
    // 播放统计统一交给 libraryModel.startPlaylistFrom / bumpPlayStatsForTrack 处理
  };

  // 操作：随机播放
  const handleShufflePlay = () => {
    if (!tracksForView.length) return;
    const cloned = tracksForView.slice();
    for (let i = cloned.length - 1; i > 0; i -= 1) {
      const j = Math.floor(Math.random() * (i + 1));
      [cloned[i], cloned[j]] = [cloned[j], cloned[i]];
    }
    startPlaylistFrom(cloned, 0);
    // 同上，不在页面层重复统计
  };

  // 操作：导入本地音乐
  const handleImport = async () => {
    if (!addTracks) {
      console.warn(
        "[LibraryPage] addTracks 不存在，暂不支持在此处导入曲目。",
      );
      return;
    }

    const selected = await open({
      multiple: true,
      directory: false,
      filters: [
        {
          name: "Audio",
          extensions: [
            "mp3",
            "flac",
            "wav",
            "ogg",
            "m4a",
            "aac",
            "opus",
            "alac",
          ],
        },
      ],
    });

    if (!selected) return;

    const now = new Date().toISOString();
    const files = Array.isArray(selected) ? selected : [selected];

    const defaultArtist = t(
      "library.import.defaultArtist",
      "未知艺人",
    );
    const defaultAlbum = t(
      "library.import.defaultAlbum",
      "未分专辑",
    );

    const newTracks: LibraryTrack[] = files.map((file) => {
      const fullPath = String(file);
      const filename = fullPath.split(/[\\/]/).pop() || fullPath;
      const dotIndex = filename.lastIndexOf(".");
      const baseTitle = dotIndex > 0 ? filename.slice(0, dotIndex) : filename;

      return {
        id: fullPath,
        filePath: fullPath,
        title: baseTitle,
        artist: defaultArtist,
        album: defaultAlbum,
        addedAt: now,
      };
    });

    // store 里实际类型是 MusicTrack，这里结构兼容，直接塞进去
    addTracks(newTracks as any);
    saveCurrentLibrary();
  };

  // 操作：清空资料库
  const handleClear = () => {
    if (!clearLibrary) {
      console.warn("[LibraryPage] clearLibrary 不存在，无法清空曲库。");
      return;
    }
    const confirmText = t(
      "library.clear.confirmMessage",
      "确定要清空资料库吗？这不会删除你的音频文件，但会清空索引。",
    );
    const ok = window.confirm(confirmText);
    if (!ok) return;
    clearLibrary();
    saveCurrentLibrary();
  };

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        display: "flex",
        flexDirection: "column",
        padding: 16,
        gap: 16,
      }}
    >
      <LibraryHeader
        totalCount={Array.isArray(tracks) ? tracks.length : 0}
        search={search}
        onSearchChange={setSearch}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        sortKey={sortKey}
        onSortKeyChange={setSortKey}
        onPlayAll={handlePlayAll}
        onShufflePlay={handleShufflePlay}
        onImport={handleImport}
        onClear={handleClear}
      />

      <div
        style={{
          flex: 1,
          minHeight: 0,
          display: "flex",
        }}
      >
        {viewMode === "tracks" ? (
          <LibraryTracksView
            tracks={tracksForView}
            onPlayTrack={(_track: LibraryTrack, index: number) => {
              // 统一通过 startPlaylistFrom 启动播放，内部会负责播放状态和统计
              startPlaylistFrom(tracksForView, index);
            }}
          />
        ) : (
          <LibraryGroupViews
            mode={viewMode === "albums" ? "albums" : "artists"}
            tracks={tracksForView}
            onPlayGroup={(groupTracks: LibraryTrack[]) => {
              if (!groupTracks.length) return;
              startPlaylistFrom(groupTracks, 0);
            }}
          />
        )}
      </div>
    </div>
  );
};

export default LibraryPage;
