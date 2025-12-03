import React from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStore } from "../store/player";
import type { PlayerTrack } from "../store/player";

export const TrackList: React.FC = () => {
  const playlist = usePlayerStore((s) => s.playlist);
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const setPlaylist = usePlayerStore((s) => s.setPlaylist);
  const addTracks = usePlayerStore((s) => s.addTracks);
  const playTrack = usePlayerStore((s) => s.playTrack);

  // 导入本地音乐文件
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

      const newTracks: PlayerTrack[] = paths.map((p, i) => {
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

      if (playlist.length === 0) {
        setPlaylist(newTracks);
      } else {
        addTracks(newTracks);
      }
    } catch (err) {
      console.error("[TrackList] 导入本地文件失败：", err);
    }
  };

  const handleRowClick = (index: number) => {
    playTrack(index);
  };

  const hasTracks = playlist.length > 0;

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
                <th className="px-3 py-2 w-12 text-right font-medium text-gray-500">
                  #
                </th>
                <th className="px-3 py-2 text-left font-medium text-gray-600">
                  标题
                </th>
                <th className="px-3 py-2 text-left font-medium text-gray-600 w-48">
                  艺人
                </th>
              </tr>
            </thead>
            <tbody>
              {playlist.map((track, index) => {
                const active = index === currentIndex;
                const title = track.title ?? "未知标题";
                const artist = track.artist ?? "未知艺人";

                return (
                  <tr
                    key={track.id ?? index}
                    onClick={() => handleRowClick(index)}
                    className={
                      "cursor-pointer select-none " +
                      (active
                        ? "bg-purple-50"
                        : "hover:bg-gray-50 transition-colors")
                    }
                  >
                    <td className="px-3 py-2 text-right text-gray-500">
                      {index + 1}
                    </td>
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
