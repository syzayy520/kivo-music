// src/pages/PlaylistPage.tsx
import React, { useMemo, useState } from "react";
import { usePlayerStore, PlayerTrack } from "../store/player";
import { useLibrary } from "../store/library";
import PlaylistHeader, {
  PlaylistTabKey,
  PlaylistTabInfo,
} from "../components/playlists/PlaylistHeader";
import PlaylistTable from "../components/playlists/PlaylistTable";
import { clearQueue } from "../playlists/playQueueModel";
import { getTrackIdentity } from "../library/libraryModel";
import { useI18n } from "../i18n";

const ONE_DAY_MS = 24 * 60 * 60 * 1000;

const PlaylistPage: React.FC = () => {
  // 当前播放队列（真实队列，给“当前队列”tab 用）
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? -1);

  // 整个曲库（给智能列表用：最近添加 / 最近播放 / 常常播放 / 喜欢的歌曲）
  const libraryTracks = useLibrary((s: any) => s.tracks ?? []);

  const [activeTab, setActiveTab] = useState<PlaylistTabKey>("queue");

  const currentTrack: PlayerTrack | null =
    Array.isArray(playlist) &&
    currentIndex >= 0 &&
    currentIndex < playlist.length
      ? (playlist[currentIndex] as PlayerTrack)
      : null;

  const { t } = useI18n();

  /**
   * 生成一个稳定的“身份 key”，用于：
   * - 行高亮（当前播放行）
   * - 在队列中查找等
   *
   * 优先使用 libraryModel.getTrackIdentity，兜底用 filePath/title。
   */
  const makeIdentityKey = (track: any): string => {
    const identity = getTrackIdentity(track as any);
    if (identity) return identity;

    const filePath = track?.filePath ?? track?.path ?? track?.location ?? "";
    const idPart =
      track?.id ?? track?.trackId ?? (filePath || track?.title || "track");

    return String(idPart);
  };

  // 通用排序工具（数值）
  const sortByDesc = <T,>(items: T[], getter: (item: T) => number): T[] => {
    return [...items].sort((a, b) => getter(b) - getter(a));
  };

  // 通用排序工具（日期字符串，越新排越前）
  const sortByDateDesc = <T,>(
    items: T[],
    getter: (item: T) => string | null | undefined,
  ): T[] => {
    return [...items].sort((a, b) => {
      const av = getter(a);
      const bv = getter(b);
      const at = av ? Date.parse(av) : 0;
      const bt = bv ? Date.parse(bv) : 0;
      return bt - at;
    });
  };

  const { tabs, tracksForView } = useMemo(() => {
    const queueTracks: PlayerTrack[] = Array.isArray(playlist)
      ? (playlist as PlayerTrack[])
      : [];

    // 智能列表优先使用整库；如果库还没加载，就退化用当前队列
    const baseForSmart: any[] =
      Array.isArray(libraryTracks) && libraryTracks.length > 0
        ? libraryTracks
        : queueTracks;

    const nowMs = Date.now();

    // 最近添加：有 addedAt 的优先，按时间降序
    const recentlyAdded = sortByDateDesc(
      baseForSmart.filter((t) => !!(t as any).addedAt),
      (t) => (t as any).addedAt ?? null,
    );

    // 最近播放：只看有 lastPlayedAt 的
    const recentlyPlayed = sortByDateDesc(
      baseForSmart.filter((t) => !!(t as any).lastPlayedAt),
      (t) => (t as any).lastPlayedAt ?? null,
    );

    // 常常播放：基于 playCount + 最近性权重
    const mostPlayed = sortByDesc(
      baseForSmart.filter((t) => {
        const count = (t as any).playCount;
        return typeof count === "number" && count > 0;
      }),
      (t) => {
        const anyT = t as any;
        const playCount =
          typeof anyT.playCount === "number" && Number.isFinite(anyT.playCount)
            ? anyT.playCount
            : 0;

        let recencyBoost = 0;
        if (anyT.lastPlayedAt) {
          const playedAt = Date.parse(anyT.lastPlayedAt);
          if (Number.isFinite(playedAt)) {
            const daysAgo = (nowMs - playedAt) / ONE_DAY_MS;
            if (daysAgo <= 30) {
              // 最近 30 天内播放过的，最多加 1 分，越近越高
              recencyBoost = 1 - Math.min(daysAgo, 30) / 30;
            }
          }
        }

        // 权重：频率为主，最近性为辅
        return playCount * 1000 + recencyBoost;
      },
    );

    // 喜欢的歌曲：favorite === true
    const favorites = baseForSmart.filter((t) => !!(t as any).favorite);

    const tabsInfo: PlaylistTabInfo[] = [
      {
        key: "queue",
        label: t("playlist.tabs.queue"),
        count: queueTracks.length,
      },
      {
        key: "recentlyAdded",
        label: t("playlist.tabs.recentlyAdded"),
        count: recentlyAdded.length,
      },
      {
        key: "recentlyPlayed",
        label: t("playlist.tabs.recentlyPlayed"),
        count: recentlyPlayed.length,
      },
      {
        key: "mostPlayed",
        label: t("playlist.tabs.mostPlayed"),
        count: mostPlayed.length,
      },
      {
        key: "favorites",
        label: t("playlist.tabs.favorites"),
        count: favorites.length,
      },
    ];

    let dataForView: PlayerTrack[];

    switch (activeTab) {
      case "recentlyAdded":
        dataForView = recentlyAdded as PlayerTrack[];
        break;
      case "recentlyPlayed":
        dataForView = recentlyPlayed as PlayerTrack[];
        break;
      case "mostPlayed":
        dataForView = mostPlayed as PlayerTrack[];
        break;
      case "favorites":
        dataForView = favorites as PlayerTrack[];
        break;
      case "queue":
      default:
        dataForView = queueTracks;
    }

    return {
      tabs: tabsInfo,
      tracksForView: dataForView,
    };
  }, [playlist, libraryTracks, activeTab, t]);

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
