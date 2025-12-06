// src/components/now-playing/UpNextPanel.tsx
import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { usePlayerStore } from "../../store/player";
import type { PlayerTrack } from "../../types/track";
import { kivoTheme } from "../../styles/theme";
import {
  clearQueue,
  playFromQueue,
  removeFromQueue,
  moveInQueue,
} from "../../playlists/playQueueModel";
import { UpNextRow } from "./UpNextRow";
import { log } from "../../utils/log";

import { useI18n } from "../../i18n";

/**
 * 接下来播放（Up Next）面板。
 *
 * 设计原则：
 * - 这里只关心“拿数据 + 组织布局 + 调用队列模型”；
 * - 单行的展示和交互细节拆到 UpNextRow，避免文件过长；
 * - 所有对队列的修改统一走 playQueueModel。
 */
export const UpNextPanel: React.FC = () => {
  const { t } = useI18n();

  const playlist = usePlayerStore(
    (s) => s.playlist as PlayerTrack[] | undefined,
  );
  const currentIndex = usePlayerStore((s) => s.currentIndex);
  const isPlaying = usePlayerStore((s) => s.isPlaying);

  // 队列为空时的占位视图
  if (!playlist || playlist.length === 0) {
    return (
      <div
        style={{
          flex: 1,
          minHeight: 0,
          borderRadius: kivoTheme.radius.lg,
          border: `1px dashed ${kivoTheme.colors.borderSubtle}`,
          background: "#ffffff",
          padding: kivoTheme.spacing.lg,
          fontSize: 13,
          color: "#9ca3af",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          textAlign: "center",
          gap: 8,
        }}
      >
        <div style={{ fontSize: 14, fontWeight: 600, color: "#111827" }}>
          {t("nowPlaying.upNext.empty.title")}
        </div>
        <div>
          {t("nowPlaying.upNext.empty.description")}
        </div>
      </div>
    );
  }

  const total = playlist.length;
  const headerTitle = t("nowPlaying.upNext.header.title");

  let statusText: string;
  if (currentIndex >= 0 && currentIndex < total) {
    const stateLabel = t(
      isPlaying
        ? "nowPlaying.info.status.playing"
        : "nowPlaying.info.status.paused",
    );
    statusText = t("nowPlaying.upNext.header.status.withIndex")
      .replace("{index}", String(currentIndex + 1))
      .replace("{total}", String(total))
      .replace("{state}", stateLabel);
  } else {
    statusText = t("nowPlaying.upNext.header.status.totalOnly").replace(
      "{total}",
      String(total),
    );
  }

  const handleClear = () => clearQueue();

  return (
    <div
      style={{
        flex: 1,
        minHeight: 0,
        borderRadius: kivoTheme.radius.lg,
        border: `1px solid ${kivoTheme.colors.borderSubtle}`,
        background: "#ffffff",
        padding: kivoTheme.spacing.lg,
        fontSize: 13,
        color: "#111827",
        display: "flex",
        flexDirection: "column",
        gap: kivoTheme.spacing.sm,
      }}
    >
      {/* 头部 */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          gap: kivoTheme.spacing.sm,
        }}
      >
        <div>
          <div
            style={{
              fontSize: 14,
              fontWeight: 600,
            }}
          >
            {headerTitle}
          </div>
          <div
            style={{
              fontSize: 12,
              color: "#6b7280",
            }}
          >
            {statusText}
          </div>
        </div>

        <button
          onClick={handleClear}
          style={{
            padding: "4px 10px",
            fontSize: 12,
            borderRadius: 9999,
            border: "1px solid #ef4444",
            background: "rgba(248,113,113,0.08)",
            color: "#b91c1c",
            cursor: "pointer",
            whiteSpace: "nowrap",
          }}
        >
          {t("nowPlaying.upNext.clearButton")}
        </button>
      </div>

      {/* 列表区域 */}
      <div
        style={{
          flex: 1,
          minHeight: 0,
          marginTop: kivoTheme.spacing.sm,
          borderRadius: kivoTheme.radius.md,
          background: "#f9fafb",
          overflow: "hidden",
          display: "flex",
          flexDirection: "column",
        }}
      >
        {/* 列头 */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            padding: "6px 10px",
            fontSize: 11,
            color: "#6b7280",
            borderBottom: "1px solid rgba(148,163,184,0.4)",
            background: "rgba(15,23,42,0.02)",
          }}
        >
          <div
            style={{
              width: 28,
              textAlign: "right",
              paddingRight: 4,
            }}
          >
            #
          </div>
          <div
            style={{
              flex: 1,
              minWidth: 0,
            }}
          >
            {t("nowPlaying.upNext.column.trackAndArtist")}
          </div>
          <div
            style={{
              minWidth: 64,
              textAlign: "right",
            }}
          >
            {t("nowPlaying.upNext.column.status")}
          </div>
          <div
            style={{
              minWidth: 88,
              textAlign: "right",
            }}
          >
            {t("nowPlaying.upNext.column.actions")}
          </div>
        </div>

        {/* 列表内容 */}
        <div
          style={{
            flex: 1,
            minHeight: 0,
            overflowY: "auto",
          }}
        >
          {playlist.map((track, index) => {
            const isCurrent = index === currentIndex;
            const isNext = index === currentIndex + 1;
            const canMoveUp = index > 0;
            const canMoveDown = index < total - 1;
            const canRemove = !isCurrent;

            const handlePlayFrom = () => playFromQueue(index);
            const handleMoveUp = () => moveInQueue(index, index - 1);
            const handleMoveDown = () => moveInQueue(index, index + 1);
            const handleRemove = () => removeFromQueue(index);
            const handleOpenFolder = () => {
              void openFolderForTrack(track);
            };

            return (
              <UpNextRow
                // 即使 filePath 一样，也用 index 拼接，保证 key 唯一
                key={buildTrackRowKey(track, index)}
                track={track}
                index={index}
                isCurrent={isCurrent}
                isNext={isNext}
                canMoveUp={canMoveUp}
                canMoveDown={canMoveDown}
                isRemovable={canRemove}
                onPlay={handlePlayFrom}
                onMoveUp={handleMoveUp}
                onMoveDown={handleMoveDown}
                onRemove={canRemove ? handleRemove : undefined}
                onOpenFolder={handleOpenFolder}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
};

function buildTrackRowKey(track: PlayerTrack, index: number): string {
  const filePath =
    (track as any).filePath ??
    (track as any).path ??
    (track as any).location ??
    "";
  const idPart =
    (track as any).id ??
    (track as any).trackId ??
    (track as any).trackID ??
    "";
  const safeId = String(idPart || index);
  return `${filePath}::${safeId}`;
}

/**
 * 提取文件所在目录
 */
function getDirectoryPathFromTrack(track: PlayerTrack): string | null {
  const fullPath =
    (track as any).filePath ??
    (track as any).path ??
    (track as any).location ??
    null;
  if (!fullPath || typeof fullPath !== "string") return null;

  const lastSep = Math.max(
    fullPath.lastIndexOf("\\"),
    fullPath.lastIndexOf("/"),
  );
  if (lastSep <= 0) return null;

  return fullPath.slice(0, lastSep);
}

/**
 * 处理 Windows 下 \\?\\UNC\\ 前缀，给 opener 用
 */
function normalizeWindowsDirPath(dirPath: string): string {
  if (dirPath.startsWith("\\\\?\\UNC\\")) {
    // \\?\\UNC\\server\\share\\path -> \\server\\share\\path
    return "\\" + dirPath.slice("\\\\?\\UNC\\".length);
  }
  if (dirPath.startsWith("\\\\?\\")) {
    // \\?\\C:\\Music -> C:\\Music
    return dirPath.slice("\\\\?\\".length);
  }
  return dirPath;
}

/**
 * 调用 tauri-plugin-opener 打开文件所在目录（统一用 open_path）
 */
async function openFolderForTrack(track: PlayerTrack): Promise<void> {
  const rawDir = getDirectoryPathFromTrack(track);
  if (!rawDir) {
    log.warn("UpNextPanel", "无法解析曲目的所在目录", { track });
    return;
  }

  const dirPath = normalizeWindowsDirPath(rawDir);

  try {
    await invoke("plugin:opener|open_path", { path: dirPath });
  } catch (err) {
    log.warn(
      "UpNextPanel",
      "调用 opener 打开文件所在目录失败（插件未配置、路径不兼容或权限不足）",
      {
        dirPath,
        err,
        track,
      },
    );
  }
}

export default UpNextPanel;
