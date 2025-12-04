// src/components/playlists/PlaylistHeader.tsx
import React from "react";
import { kivoTheme } from "../../styles/theme";

export type PlaylistTabKey =
  | "queue"
  | "recentlyAdded"
  | "recentlyPlayed"
  | "mostPlayed"
  | "favorites";

export interface PlaylistTabInfo {
  key: PlaylistTabKey;
  label: string;
  count: number;
}

interface PlaylistHeaderProps {
  activeTab: PlaylistTabKey;
  tabs: PlaylistTabInfo[];
  onChangeTab: (key: PlaylistTabKey) => void;
  onClearQueue: () => void;
  currentTrack: any | null;
}

const { radius, spacing } = kivoTheme;

const PlaylistHeader: React.FC<PlaylistHeaderProps> = ({
  activeTab,
  tabs,
  onChangeTab,
  onClearQueue,
  currentTrack,
}) => {
  const totalCount =
    tabs.find((t) => t.key === "queue")?.count ?? 0;

  const description = `根据“当前队列 / 最近添加 / 最近播放 / 最常播放 / 喜欢的歌曲”等模式，从当前曲目集合生成智能播放列表。目前共有 ${totalCount} 首曲目可供排列组合。`;

  const currentTitle =
    currentTrack?.title ??
    currentTrack?.name ??
    currentTrack?.fileName ??
    "暂无正在播放";

  const currentArtist = currentTrack?.artist ?? "未知艺人";

  return (
    <div
      style={{
        borderRadius: radius.xl,
        padding: 24,
        background:
          "linear-gradient(135deg, rgba(59,130,246,1) 0%, rgba(96,165,250,1) 35%, rgba(56,189,248,1) 100%)",
        color: "#ffffff",
        boxShadow: "0 16px 40px rgba(15,23,42,0.35)",
        display: "flex",
        flexDirection: "column",
        gap: 16,
      }}
    >
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          gap: 16,
        }}
      >
        <div style={{ flex: 1, minWidth: 0 }}>
          <div
            style={{
              fontSize: 22,
              fontWeight: 700,
              marginBottom: 6,
            }}
          >
            播放列表 & 智能列表
          </div>
          <div
            style={{
              fontSize: 13,
              opacity: 0.9,
              lineHeight: 1.6,
            }}
          >
            {description}
          </div>
        </div>

        <div
          style={{
            minWidth: 260,
            padding: 14,
            borderRadius: 20,
            background: "rgba(15,23,42,0.25)",
            display: "flex",
            flexDirection: "column",
            gap: 4,
          }}
        >
          <div
            style={{
              fontSize: 12,
              opacity: 0.85,
            }}
          >
            正在播放：
          </div>
          <div
            style={{
              fontSize: 14,
              fontWeight: 600,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {currentTitle}
          </div>
          <div
            style={{
              fontSize: 12,
              opacity: 0.85,
              whiteSpace: "nowrap",
              overflow: "hidden",
              textOverflow: "ellipsis",
            }}
          >
            {currentArtist}
          </div>
        </div>
      </div>

      {/* Tabs + 操作区 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: spacing.md,
          marginTop: 4,
        }}
      >
        <div
          style={{
            display: "flex",
            gap: 8,
            flexWrap: "wrap",
          }}
        >
          {tabs.map((tab) => {
            const isActive = tab.key === activeTab;
            return (
              <button
                key={tab.key}
                onClick={() => onChangeTab(tab.key)}
                style={{
                  padding: "6px 14px",
                  fontSize: 12,
                  borderRadius: 9999,
                  borderWidth: 1,
                  borderStyle: "solid",
                  borderColor: isActive
                    ? "rgba(255,255,255,0.95)"
                    : "rgba(191,219,254,0.7)",
                  background: isActive
                    ? "rgba(15,23,42,0.16)"
                    : "rgba(15,23,42,0.08)",
                  color: "#ffffff",
                  cursor: "pointer",
                  display: "flex",
                  alignItems: "center",
                  gap: 6,
                  whiteSpace: "nowrap",
                }}
              >
                <span>{tab.label}</span>
                <span
                  style={{
                    fontSize: 11,
                    padding: "0 6px",
                    borderRadius: 9999,
                    background: "rgba(15,23,42,0.35)",
                  }}
                >
                  {tab.count}
                </span>
              </button>
            );
          })}
        </div>

        <div style={{ flex: 1 }} />

        <button
          onClick={onClearQueue}
          style={{
            padding: "6px 12px",
            fontSize: 12,
            borderRadius: 9999,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "rgba(248,113,113,0.95)",
            background: "rgba(254,242,242,0.08)",
            color: "#fee2e2",
            cursor: "pointer",
            whiteSpace: "nowrap",
          }}
        >
          清空当前播放队列
        </button>
      </div>
    </div>
  );
};

export default PlaylistHeader;
