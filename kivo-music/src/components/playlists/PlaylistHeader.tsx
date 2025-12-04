// src/components/playlists/PlaylistHeader.tsx
import React from "react";
import type { PlayerTrack } from "../../store/player";
import { useKivoTheme } from "../../styles/ThemeContext";
import { KivoButton } from "../common/KivoButton";

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

export interface PlaylistHeaderProps {
  /** 当前激活的 Tab */
  activeTab: PlaylistTabKey;
  /** Tab 列表（包含 key / label / count） */
  tabs: PlaylistTabInfo[];
  /** 切换 Tab 时回调 */
  onChangeTab: (tab: PlaylistTabKey) => void;
  /** 清空当前播放队列 */
  onClearQueue: () => void;
  /** 当前正在播放的曲目（可选） */
  currentTrack?: PlayerTrack | null;
}

/**
 * 播放列表 & 智能列表头部
 *
 * - 显示当前曲目总数说明文案
 * - 展示正在播放的小卡片
 * - 提供 Tab 切换（当前队列 / 最近添加 / 最近播放 / 常常播放 / 喜欢的歌曲）
 * - 提供“清空当前播放队列”操作
 *
 * 注意：
 * - 所有颜色、圆角、间距统一从 theme 中取；
 * - 样式中只使用 backgroundColor，不与 background 混用。
 */
const PlaylistHeader: React.FC<PlaylistHeaderProps> = ({
  activeTab,
  tabs,
  onChangeTab,
  onClearQueue,
  currentTrack,
}) => {
  const { theme } = useKivoTheme();
  const { colors, radius, spacing } = theme;

  const totalCount = tabs.find((t) => t.key === "queue")?.count ?? 0;

  const description = `根据“当前队列 / 最近添加 / 最近播放 / 常常播放 / 喜欢的歌曲”等模式，从当前曲目集合生成智能播放列表。目前共有 ${totalCount} 首曲目可供排列组合。`;

  const currentTitle =
    currentTrack?.title ||
    // 兼容可能存在的 name 字段
    // @ts-expect-error 兼容旧数据结构
    currentTrack?.name ||
    "当前没有正在播放的歌曲";

  const currentArtist =
    currentTrack?.artist ||
    // @ts-expect-error 兼容旧数据结构
    currentTrack?.artistName ||
    "选择一首歌开始播放，或从任意列表中双击添加到队列";

  const containerStyle: React.CSSProperties = {
    padding: spacing.lg,
    borderBottomWidth: 1,
    borderBottomStyle: "solid",
    borderBottomColor: colors.borderSubtle,
    display: "flex",
    flexDirection: "column",
    gap: spacing.md,
  };

  const heroStyle: React.CSSProperties = {
    display: "flex",
    flexDirection: "row",
    alignItems: "stretch",
    gap: spacing.lg,
    padding: spacing.lg,
    borderRadius: radius.xl,
    backgroundColor: "#0f172a", // 深底色
    backgroundImage:
      "radial-gradient(circle at 0% 0%, rgba(59,130,246,0.6), transparent 55%), radial-gradient(circle at 120% 120%, rgba(56,189,248,0.35), transparent 55%)",
    backgroundRepeat: "no-repeat",
    backgroundSize: "cover",
    color: "#e5e7eb",
  };

  const titleStyle: React.CSSProperties = {
    fontSize: 22,
    fontWeight: 650,
    letterSpacing: 0.3,
    marginBottom: 4,
  };

  const descStyle: React.CSSProperties = {
    fontSize: 13,
    color: "rgba(226,232,240,0.86)",
    maxWidth: 520,
    lineHeight: 1.6,
  };

  const currentCardStyle: React.CSSProperties = {
    minWidth: 220,
    maxWidth: 280,
    padding: spacing.md,
    borderRadius: radius.lg,
    backgroundColor: "rgba(15,23,42,0.82)",
    borderWidth: 1,
    borderStyle: "solid",
    borderColor: "rgba(148,163,184,0.55)",
    display: "flex",
    flexDirection: "column",
    gap: 6,
  };

  const currentTitleStyle: React.CSSProperties = {
    fontSize: 14,
    fontWeight: 600,
    whiteSpace: "nowrap",
    overflow: "hidden",
    textOverflow: "ellipsis",
  };

  const currentArtistStyle: React.CSSProperties = {
    fontSize: 12,
    whiteSpace: "nowrap",
    overflow: "hidden",
    textOverflow: "ellipsis",
    color: "rgba(148,163,184,0.95)",
  };

  const badgeStyle: React.CSSProperties = {
    alignSelf: "flex-start",
    fontSize: 11,
    padding: "2px 8px",
    borderRadius: radius.pill,
    backgroundColor: "rgba(15,23,42,0.75)",
    borderWidth: 1,
    borderStyle: "solid",
    borderColor: "rgba(148,163,184,0.6)",
  };

  const tabsRowStyle: React.CSSProperties = {
    marginTop: spacing.md,
    display: "flex",
    alignItems: "center",
    gap: spacing.md,
  };

  const tabsContainerStyle: React.CSSProperties = {
    display: "flex",
    alignItems: "center",
    gap: spacing.sm,
    padding: 4,
    borderRadius: radius.pill,
    backgroundColor: "rgba(15,23,42,0.04)",
  };

  return (
    <header style={containerStyle}>
      {/* 顶部智能列表说明 + 当前播放卡片 */}
      <div style={heroStyle}>
        <div style={{ flex: 1, minWidth: 0, display: "flex", flexDirection: "column", gap: 8 }}>
          <div style={titleStyle}>播放列表 & 智能列表</div>
          <div style={descStyle}>{description}</div>
          <div
            style={{
              marginTop: 6,
              fontSize: 12,
              color: "rgba(191,219,254,0.95)",
            }}
          >
            智能列表会优先考虑你的最近播放、播放次数和“喜欢的歌曲”等行为数据。
          </div>
        </div>

        <div style={currentCardStyle}>
          <span style={badgeStyle}>正在播放</span>
          <div style={currentTitleStyle}>{currentTitle}</div>
          <div style={currentArtistStyle}>{currentArtist}</div>
        </div>
      </div>

      {/* 底部：Tab 和“清空当前播放队列”按钮 */}
      <div style={tabsRowStyle}>
        <div style={tabsContainerStyle}>
          {tabs.map((tab) => {
            const isActive = tab.key === activeTab;
            return (
              <button
                key={tab.key}
                type="button"
                onClick={() => onChangeTab(tab.key)}
                style={{
                  padding: "6px 14px",
                  fontSize: 12,
                  borderRadius: 9999,
                  borderWidth: 1,
                  borderStyle: "solid",
                  borderColor: isActive
                    ? "rgba(37,99,235,0.95)"
                    : "rgba(148,163,184,0.75)",
                  backgroundColor: isActive
                    ? "rgba(37,99,235,0.18)"
                    : "rgba(15,23,42,0.02)",
                  color: isActive ? colors.textOnPrimary : colors.textOnLight,
                  display: "inline-flex",
                  alignItems: "center",
                  gap: 6,
                  cursor: "pointer",
                  whiteSpace: "nowrap",
                }}
              >
                <span>{tab.label}</span>
                <span
                  style={{
                    fontSize: 11,
                    padding: "0 6px",
                    borderRadius: 9999,
                    backgroundColor: "rgba(15,23,42,0.18)",
                    color: colors.textOnLight,
                  }}
                >
                  {tab.count}
                </span>
              </button>
            );
          })}
        </div>

        <div style={{ flex: 1 }} />

        {/* 清空队列按钮 */}
        <KivoButton variant="danger" size="sm" onClick={onClearQueue}>
          清空当前播放队列
        </KivoButton>
      </div>
    </header>
  );
};

export default PlaylistHeader;
