// src/components/TrackList.tsx
import React, { useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import {
  loadLibrary,
  saveLibrary,
} from "../persistence/LibraryPersistence";

// 为了运行优先，这里用 any 兜底，避免 TS 报错卡住编译
type AnyTrack = any;

export const TrackList: React.FC = () => {
  // ✅ 每个字段单独 selector，避免 useSyncExternalStore 的 infinite loop 提示
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);
  const setPlaylist = usePlayerStore((s: any) => s.setPlaylist);
  const playTrack =
    usePlayerStore((s: any) => s.playTrack ?? s.play) ?? (() => {});

  // 👉 启动时，从磁盘加载一次资料库
  useEffect(() => {
    let cancelled = false;

    (async () => {
      try {
        const tracks = await loadLibrary();
        if (!cancelled && tracks && tracks.length > 0) {
          setPlaylist(tracks as AnyTrack[]);
          console.info(
            "[TrackList] loaded library from disk, tracks:",
            tracks.length,
          );
        }
      } catch (err) {
        console.error("[TrackList] loadLibrary in TrackList failed:", err);
      }
    })();

    return () => {
      cancelled = true;
    };
  }, [setPlaylist]);

  // 👉 导入本地文件 + 持久化到 JSON
  const handleImportClick = async () => {
    try {
      const result = await open({
        multiple: true,
        filters: [
          {
            name: "Audio",
            extensions: ["mp3", "flac", "wav", "m4a", "ogg"],
          },
        ],
      });

      if (!result) return;

      const paths = Array.isArray(result) ? result : [result];

      const newTracks: AnyTrack[] = paths.map((p, i) => {
        const path = String(p);
        const parts = path.split(/[\\/]/);
        const filename = parts[parts.length - 1] || "未知文件";
        const title = filename.replace(/\.[^.]+$/, "");

        return {
          id: `${Date.now()}-${i}`,
          title,
          artist: "未知艺人",
          filePath: path,
        };
      });

      const merged: AnyTrack[] = [...(playlist || []), ...newTracks];

      // 更新播放器状态
      setPlaylist(merged as AnyTrack[]);

      // 写入磁盘（失败也只是打印，不影响播放）
      try {
        await saveLibrary(merged as any);
        console.info(
          "[TrackList] saveLibrary ok, tracks:",
          merged.length,
        );
      } catch (err) {
        console.error("[TrackList] saveLibrary failed:", err);
      }
    } catch (err) {
      console.error("[TrackList] 导入本地文件失败：", err);
    }
  };

  const handleRowClick = (index: number) => {
    try {
      playTrack(index);
    } catch (err) {
      console.error("[TrackList] playTrack 调用失败：", err);
    }
  };

  const hasTracks = playlist && playlist.length > 0;

  return (
    <div className="p-4 flex flex-col gap-4">
      <div>
        <button
          onClick={handleImportClick}
          className="px-3 py-1.5 rounded bg-purple-600 text-white text-sm hover:bg-purple-700"
        >
          + 导入本地音乐文件
        </button>
      </div>

      {!hasTracks ? (
        <div className="text-sm text-gray-500">
          当前还没有任何歌曲，请先点击上方按钮导入本地音乐文件。
        </div>
      ) : (
        <div className="border border-gray-200 rounded overflow-hidden text-sm">
          <table className="w-full border-collapse">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-3 py-2 text-left w-12">#</th>
                <th className="px-3 py-2 text-left">标题</th>
                <th className="px-3 py-2 text-left w-40">艺人</th>
              </tr>
            </thead>
            <tbody>
              {playlist.map((track: AnyTrack, index: number) => {
                const active = index === currentIndex;
                const title = track?.title ?? "未知标题";
                const artist = track?.artist ?? "未知艺人";

                return (
                  <tr
                    key={track?.id ?? index}
                    onClick={() => handleRowClick(index)}
                    className={
                      "cursor-pointer select-none " +
                      (active ? "bg-blue-50" : "hover:bg-gray-50")
                    }
                  >
                    <td className="px-3 py-2 text-gray-500">{index + 1}</td>
                    <td className="px-3 py-2">
                      <span className={active ? "font-semibold" : ""}>
                        {title}
                      </span>
                    </td>
                    <td className="px-3 py-2 text-gray-500">{artist}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default TrackList;
