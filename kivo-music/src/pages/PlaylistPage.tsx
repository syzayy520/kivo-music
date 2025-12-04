// src/pages/PlaylistPage.tsx
import React, { useCallback, useMemo } from "react";
import { usePlayerStore } from "../store/player";
import { PlaylistHeader } from "../components/playlists/PlaylistHeader";
import { PlaylistTable } from "../components/playlists/PlaylistTable";

/**
 * 当前播放列表页面（页面壳）：
 * - 从 player store 读取 playlist / currentIndex
 * - 绑定“清空播放列表 / 双击播放”行为
 * - UI 细节交给子组件处理
 */
const PlaylistPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);

  const playTrack = usePlayerStore(
    (s: any) => s.playTrack ?? (() => {}),
  ) as (index: number) => void;

  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  ) as (tracks: any[]) => void;

  const total = useMemo(
    () => (Array.isArray(playlist) ? playlist.length : 0),
    [playlist],
  );
  const hasTracks = total > 0;

  const handleRowDoubleClick = useCallback(
    (index: number) => {
      if (!hasTracks) return;
      playTrack(index);
    },
    [hasTracks, playTrack],
  );

  const handleClearPlaylist = useCallback(() => {
    if (!hasTracks) return;
    // 直接把播放列表设为空，player store 会把 currentIndex 重置为 -1
    setPlaylist([]);
  }, [hasTracks, setPlaylist]);

  return (
    <div
      style={{
        height: "100%",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <PlaylistHeader
        total={total}
        currentIndex={currentIndex}
        hasTracks={hasTracks}
        onClearPlaylist={handleClearPlaylist}
      />

      <PlaylistTable
        playlist={playlist}
        currentIndex={currentIndex}
        onRowDoubleClick={handleRowDoubleClick}
      />

      <div
        style={{
          marginTop: 8,
          fontSize: 11,
          color: "#9ca3af",
        }}
      >
        提示：双击任意一行可以从该位置开始播放。后续版本会在此基础上增加「拖动排序 /
        从队列中移除 / 保存为歌单」等高级功能。
      </div>
    </div>
  );
};

export default PlaylistPage;
