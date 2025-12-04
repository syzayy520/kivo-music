// src/components/library/LibraryHeader.tsx
import React from "react";
import { PageHeader } from "../layout/PageHeader";
import { KivoButton } from "../common/KivoButton";
import { kivoTheme } from "../../styles/theme";
import type { SortKey } from "../../library/libraryModel";

export type LibraryViewMode = "tracks" | "albums" | "artists";

export interface LibraryHeaderProps {
  /** 曲目总数：可以传 totalCount 或 totalTracks，两个都传则以 totalTracks 为准 */
  totalCount?: number;
  totalTracks?: number;

  viewMode: LibraryViewMode;
  sortKey: SortKey;

  /** 搜索关键字：可以是 search 或 searchText */
  search?: string;
  searchText?: string;

  onViewModeChange: (mode: LibraryViewMode) => void;
  onSortKeyChange: (key: SortKey) => void;

  /** 搜索变化：支持 onSearchChange / onSearchTextChange 任意一个 */
  onSearchChange?: (text: string) => void;
  onSearchTextChange?: (text: string) => void;

  onPlayAll: () => void;
  onShufflePlay: () => void;

  /** 导入相关：任意一个有值即可 */
  onImport?: () => void;
  onImportLocal?: () => void;
  onImportTracks?: () => void;
  onImportLocalTracks?: () => void;

  /** 清空相关：任意一个有值即可 */
  onClearLibrary?: () => void;
  onClearAll?: () => void;
  onClear?: () => void;
}

const LibraryHeader: React.FC<LibraryHeaderProps> = (props) => {
  const {
    totalCount,
    totalTracks,
    viewMode,
    sortKey,
    search,
    searchText,
    onViewModeChange,
    onSortKeyChange,
    onSearchChange,
    onSearchTextChange,
    onPlayAll,
    onShufflePlay,
    onImport,
    onImportLocal,
    onImportTracks,
    onImportLocalTracks,
    onClearLibrary,
    onClearAll,
    onClear,
  } = props;

  const { colors, radius, spacing } = kivoTheme;

  // 统一曲目数量
  const total =
    typeof totalTracks === "number"
      ? totalTracks
      : typeof totalCount === "number"
      ? totalCount
      : 0;

  // 统一搜索 value
  const searchValue = typeof searchText === "string" ? searchText : search ?? "";

  // 统一搜索 change
  const handleSearchChange = (value: string) => {
    if (onSearchTextChange) return onSearchTextChange(value);
    if (onSearchChange) return onSearchChange(value);
    console.warn(
      "[LibraryHeader] 没有传入搜索回调（onSearchChange / onSearchTextChange），输入不会影响结果。",
    );
  };

  // 统一导入点击
  const handleImportClick = () => {
    if (onImport) return onImport();
    if (onImportLocal) return onImportLocal();
    if (onImportTracks) return onImportTracks();
    if (onImportLocalTracks) return onImportLocalTracks();
    console.warn(
      "[LibraryHeader] 没有传入导入回调（onImport / onImportLocal / onImportTracks / onImportLocalTracks）。",
    );
  };

  // 统一清空点击
  const handleClearClick = () => {
    if (onClearLibrary) return onClearLibrary();
    if (onClearAll) return onClearAll();
    if (onClear) return onClear();
    console.warn(
      "[LibraryHeader] 没有传入清空回调（onClearLibrary / onClearAll / onClear）。",
    );
  };

  const viewButton = (mode: LibraryViewMode, label: string) => {
    const isActive = viewMode === mode;
    return (
      <button
        key={mode}
        onClick={() => onViewModeChange(mode)}
        style={{
          flex: 1,
          paddingTop: spacing.sm,
          paddingBottom: spacing.sm,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: isActive
            ? "rgba(15, 23, 42, 0.1)"
            : "rgba(255, 255, 255, 0.18)",
          backgroundColor: isActive
            ? "rgba(15, 23, 42, 0.16)"
            : "rgba(15, 23, 42, 0.08)",
          color: "#f9fafb",
          fontSize: 13,
          fontWeight: isActive ? 600 : 500,
          cursor: "pointer",
          borderRadius: radius.pill,
          boxShadow: isActive
            ? "0 0 0 1px rgba(15, 23, 42, 0.3)"
            : "none",
          transition:
            "background-color 120ms ease-out, border-color 120ms ease-out, box-shadow 120ms ease-out",
        }}
      >
        {label}
      </button>
    );
  };

  // 注意：SortKey 在 libraryModel 里定义为 "default" | "title" | "artist" | "album" | "recent"
  const sortLabelMap: Record<SortKey, string> = {
    default: "默认顺序",
    title: "标题",
    artist: "艺人",
    album: "专辑",
    recent: "最近播放",
  };

  const centerSlot = (
    <>
      {/* 左：搜索框 */}
      <div
        style={{
          flex: 2,
          minWidth: 280,
          display: "flex",
          alignItems: "center",
          gap: spacing.sm,
        }}
      >
        <input
          placeholder="搜索歌曲 / 艺人 / 专辑 / 路径..."
          value={searchValue}
          onChange={(e) => handleSearchChange(e.target.value)}
          style={{
            flex: 1,
            height: 32,
            borderRadius: radius.pill,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "rgba(15, 23, 42, 0.18)",
            paddingLeft: spacing.lg,
            paddingRight: spacing.lg,
            fontSize: 13,
            outline: "none",
            color: colors.textOnPrimary,
            backgroundColor: "rgba(15, 23, 42, 0.16)",
          }}
        />
      </div>

      {/* 中：视图切换 */}
      <div
        style={{
          flex: 1.6,
          minWidth: 260,
          display: "flex",
          alignItems: "center",
          gap: spacing.sm,
          backgroundColor: "rgba(15, 23, 42, 0.30)",
          borderRadius: radius.pill,
          padding: 3,
        }}
      >
        {viewButton("tracks", "按歌曲")}
        {viewButton("albums", "按专辑")}
        {viewButton("artists", "按艺人")}
      </div>

      {/* 右：排序选择 */}
      <div
        style={{
          flex: 1,
          minWidth: 180,
          display: "flex",
          justifyContent: "flex-end",
        }}
      >
        <select
          value={sortKey}
          onChange={(e) => onSortKeyChange(e.target.value as SortKey)}
          style={{
            minWidth: 150,
            height: 32,
            borderRadius: radius.pill,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "rgba(15, 23, 42, 0.2)",
            paddingLeft: spacing.md,
            paddingRight: spacing.md,
            fontSize: 13,
            color: colors.textOnPrimary,
            backgroundColor: "rgba(15, 23, 42, 0.16)",
          }}
        >
          {(Object.keys(sortLabelMap) as SortKey[]).map((key) => (
            <option key={key} value={key}>
              {sortLabelMap[key]}
            </option>
          ))}
        </select>
      </div>
    </>
  );

  const rightSlot = (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "flex-end",
        gap: spacing.sm,
      }}
    >
      <div style={{ display: "flex", gap: spacing.sm }}>
        <KivoButton onClick={onPlayAll}>播放全部</KivoButton>
        <KivoButton variant="secondary" onClick={onShufflePlay}>
          随机播放
        </KivoButton>
      </div>
      <div style={{ display: "flex", gap: spacing.sm }}>
        <KivoButton
          variant="secondary"
          size="sm"
          onClick={handleImportClick}
        >
          + 导入本地音乐
        </KivoButton>
        <KivoButton
          variant="danger"
          size="sm"
          onClick={handleClearClick}
        >
          清空资料库
        </KivoButton>
      </div>
    </div>
  );

  const subtitle = (
    <>
      按照本地曲库整理你的音乐。
      支持按歌曲 / 专辑 / 艺人视图切换，支持搜索、排序和播放队列。
    </>
  );

  const extra = <>共 {total} 首已扫描的本地曲目</>;

  return (
    <PageHeader
      title="本地音乐资料库"
      subtitle={subtitle}
      leftBottomExtra={extra}
      centerSlot={centerSlot}
      rightSlot={rightSlot}
    />
  );
};

export { LibraryHeader };
export default LibraryHeader;
