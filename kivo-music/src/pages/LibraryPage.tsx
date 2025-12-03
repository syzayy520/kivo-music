import React, { useMemo, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useLibrary } from "../store/library";
import { usePlayer } from "../store/player";
import type { MusicTrack } from "../types";
import { TrackList } from "../components/TrackList";

function pathToTitle(path: string): string {
  const parts = path.split(/[/\\]/);
  const file = parts[parts.length - 1] || path;
  const dotIndex = file.lastIndexOf(".");
  return dotIndex > 0 ? file.slice(0, dotIndex) : file;
}

type SortKey = "none" | "title" | "artist";

export const LibraryPage: React.FC = () => {
  const { tracks, addTracks, clearLibrary } = useLibrary();

  const player = usePlayer();

  // 搜索关键词
  const [keyword, setKeyword] = useState("");
  // 排序字段 & 顺序
  const [sortKey, setSortKey] = useState<SortKey>("none");
  const [sortAsc, setSortAsc] = useState(true);

  const handleImport = async () => {
    try {
      const result = await open({
        multiple: true,
        filters: [
          {
            name: "Music",
            extensions: ["mp3", "flac", "wav", "ogg", "m4a"],
          },
        ],
      });

      if (!result) return;

      const paths = Array.isArray(result) ? result : [result];

      const newTracks: MusicTrack[] = paths.map((p) => ({
        id: p,
        path: p,
        title: pathToTitle(p),
        artist: "未知艺人",
        album: "未知专辑",
        duration: 0,
      }));

      addTracks(newTracks);

      // 同步到播放器队列
      const allTracks = useLibrary.getState().tracks;
      player.setQueue(allTracks);
    } catch (e) {
      console.error("导入音乐文件失败:", e);
    }
  };

  // 播放指定歌曲：根据 id 在全库中找下标
  const handlePlayTrack = (track: MusicTrack) => {
    const allTracks = useLibrary.getState().tracks;
    if (allTracks.length === 0) return;

    const index = allTracks.findIndex((t) => t.id === track.id);
    if (index === -1) return;

    player.setQueue(allTracks);
    player.playAt(index);
  };

  // 切换排序
  const toggleSort = (key: SortKey) => {
    if (key === "none") {
      setSortKey("none");
      return;
    }
    if (sortKey === key) {
      setSortAsc(!sortAsc);
    } else {
      setSortKey(key);
      setSortAsc(true);
    }
  };

  // 根据搜索 + 排序得到最终要展示的 tracks
  const displayedTracks = useMemo(() => {
    let list = tracks;

    const kw = keyword.trim().toLowerCase();
    if (kw) {
      list = list.filter((t) => {
        return (
          t.title.toLowerCase().includes(kw) ||
          t.artist.toLowerCase().includes(kw) ||
          t.album.toLowerCase().includes(kw)
        );
      });
    }

    if (sortKey !== "none") {
      list = [...list].sort((a, b) => {
        const aVal =
          sortKey === "title"
            ? a.title.toLowerCase()
            : a.artist.toLowerCase();
        const bVal =
          sortKey === "title"
            ? b.title.toLowerCase()
            : b.artist.toLowerCase();
        if (aVal === bVal) return 0;
        const res = aVal < bVal ? -1 : 1;
        return sortAsc ? res : -res;
      });
    }

    return list;
  }, [tracks, keyword, sortKey, sortAsc]);

  const total = tracks.length;
  const filteredCount = displayedTracks.length;

  const sortLabel = (key: SortKey) => {
    if (sortKey !== key) return "";
    return sortAsc ? "▲" : "▼";
  };

  return (
    <div className="space-y-4">
      {/* 顶部标题 + 搜索框 */}
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold mb-1">
            本地音乐资料库
          </h1>
          <p className="text-xs text-gray-500">
            共 {total} 首歌曲
            {keyword && ` · 匹配到 ${filteredCount} 首`}
          </p>
        </div>

        <div className="flex items-center gap-2">
          <input
            value={keyword}
            onChange={(e) => setKeyword(e.target.value)}
            placeholder="搜索标题 / 艺人 / 专辑"
            className="border border-gray-300 rounded-md px-2 py-1 text-sm w-60 focus:outline-none focus:ring-1 focus:ring-purple-400 focus:border-purple-400"
          />
        </div>
      </div>

            {/* 导入按钮 + 清空按钮 */}
      <div className="flex items-center gap-2">
        <button
          onClick={handleImport}
          className="inline-flex items-center px-3 py-1.5 rounded-md bg-purple-500 text-white text-sm hover:bg-purple-600"
        >
          + 导入本地音乐文件
        </button>

        <button
          onClick={clearLibrary}
          className="inline-flex items-center px-3 py-1.5 rounded-md border border-gray-300 text-sm text-gray-600 hover:bg-gray-50"
        >
          清空列表
        </button>
      </div>


      {!tracks.length && (
        <p className="mt-3 text-sm text-gray-500">
          还没有导入任何歌曲。可以先点击上面的「导入本地音乐文件」来选择几首歌。
        </p>
      )}

      {tracks.length > 0 && (
        <div className="mt-4 space-y-1">
          {/* 表头 */}
          <div className="flex text-xs text-gray-500 mb-1 px-3">
            <div className="w-6 text-right pr-1">#</div>
            <button
              className="flex-1 text-left hover:text-purple-600"
              onClick={() => toggleSort("title")}
            >
              标题 {sortLabel("title")}
            </button>
            <button
              className="w-32 text-right hover:text-purple-600"
              onClick={() => toggleSort("artist")}
            >
              艺人 {sortLabel("artist")}
            </button>
          </div>

          {/* 虚拟列表 */}
          <TrackList
            tracks={displayedTracks}
            onPlay={handlePlayTrack}
          />
        </div>
      )}
    </div>
  );
};
