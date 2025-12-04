// src/pages/PlaylistPage.tsx
import React, { useMemo, useState } from "react";
import { usePlayerStore, PlayerTrack } from "../store/player";
import PlaylistHeader, {
  PlaylistTabKey,
  PlaylistTabInfo,
} from "../components/playlists/PlaylistHeader";
import PlaylistTable from "../components/playlists/PlaylistTable";
import { clearQueue } from "../playlists/playQueueModel";

const PlaylistPage: React.FC = () => {
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);

  const [activeTab, setActiveTab] = useState<PlaylistTabKey>("queue");

  const currentTrack: PlayerTrack | null =
    Array.isArray(playlist) &&
    currentIndex >= 0 &&
    currentIndex < playlist.length
      ? (playlist[currentIndex] as PlayerTrack)
      : null;

  const makeIdentityKey = (track: any): string => {
    const filePath = track?.filePath ?? track?.path ?? track?.location ?? "";
    const idPart =
      track?.id ?? track?.trackId ?? (filePath || track?.title || "track");
    return String(idPart);
  };

  const sortByDesc = <T,>(items: T[], getter: (item: T) => number): T[] => {
    return [...items].sort((a, b) => getter(b) - getter(a));
  };

  const sortByDateDesc = <T,>(items: T[], getter: (item: T) => string | null | undefined): T[] => {
    return [...items].sort((a, b) => {
      const av = getter(a);
      const bv = getter(b);
      const at = av ? new Date(av).getTime() : 0;
      const bt = bv ? new Date(bv).getTime() : 0;
      return bt - at;
    });
  };

  const { tabs, tracksForView } = useMemo(() => {
    const list: PlayerTrack[] = Array.isArray(playlist)
      ? (playlist as PlayerTrack[])
      : [];

    const queueTracks = list;

    const recentlyAdded = sortByDateDesc(list, (t) => (t as any).addedAt ?? null);
    const recentlyPlayed = sortByDateDesc(
      list,
      (t) => (t as any).lastPlayedAt ?? null,
    );
    const mostPlayed = sortByDesc(list, (t) => (t as any).playCount ?? 0);
    const favorites = list.filter((t) => !!(t as any).favorite);

    const tabsInfo: PlaylistTabInfo[] = [
      { key: "queue", label: "当前队列", count: queueTracks.length },
      { key: "recentlyAdded", label: "最近添加", count: recentlyAdded.length },
      { key: "recentlyPlayed", label: "最近播放", count: recentlyPlayed.length },
      { key: "mostPlayed", label: "常常播放", count: mostPlayed.length },
      { key: "favorites", label: "喜欢的歌曲", count: favorites.length },
    ];

    let dataForView: PlayerTrack[];
    switch (activeTab) {
      case "recentlyAdded":
        dataForView = recentlyAdded;
        break;
      case "recentlyPlayed":
        dataForView = recentlyPlayed;
        break;
      case "mostPlayed":
        dataForView = mostPlayed;
        break;
      case "favorites":
        dataForView = favorites;
        break;
      case "queue":
      default:
        dataForView = queueTracks;
    }

    return {
      tabs: tabsInfo,
      tracksForView: dataForView,
    };
  }, [playlist, activeTab]);

  const handleChangeTab = (key: PlaylistTabKey) => {
    setActiveTab(key);
  };

  const handleClearQueue = () => {
    clearQueue();
  };

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        display: "flex",
        flexDirection: "column",
        padding: 24,
        gap: 16,
      }}
    >
      <PlaylistHeader
        activeTab={activeTab}
        tabs={tabs}
        onChangeTab={handleChangeTab}
        onClearQueue={handleClearQueue}
        currentTrack={currentTrack}
      />

      <div
        style={{
          flex: 1,
          minHeight: 0,
        }}
      >
        <PlaylistTable
          tracks={tracksForView}
          currentIndex={currentIndex}
          makeIdentityKey={makeIdentityKey}
        />
      </div>
    </div>
  );
};

export default PlaylistPage;
