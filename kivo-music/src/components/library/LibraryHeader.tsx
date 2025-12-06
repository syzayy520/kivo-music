// src/components/library/LibraryHeader.tsx
import React from "react";
import { PageHeader } from "../layout/PageHeader";
import { KivoButton } from "../common/KivoButton";
import type { SortKey } from "../../library/libraryModel";
import { useKivoTheme } from "../../styles/ThemeContext";
import { useI18n } from "../../i18n";

export type LibraryViewMode = "tracks" | "albums" | "artists";

export interface LibraryHeaderProps {
  /** 曲目总数：可以传 totalCount 或 totalTracks，两个都传则以 totalTracks 为准 */
  totalCount?: number;
  totalTracks?: number;

  /** 当前视图模式 */
  viewMode: LibraryViewMode;
  /** 当前排序方式 */
  sortKey: SortKey;

  /** 搜索关键字（推荐使用 search） */
  search?: string;
  /** 兼容旧版本命名 */
  searchText?: string;

  /** 搜索变更回调（推荐 onSearchChange） */
  onSearchChange?: (value: string) => void;
  /** 兼容旧命名：onSearchTextChange */
  onSearchTextChange?: (value: string) => void;

  /** 视图切换回调 */
  onViewModeChange: (mode: LibraryViewMode) => void;
  /** 排序变更回调 */
  onSortKeyChange: (key: SortKey) => void;

  /** 播放控制 */
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

const viewModeLabelKeyMap: Record<LibraryViewMode, string> = {
  tracks: "library.header.viewMode.tracks",
  albums: "library.header.viewMode.albums",
  artists: "library.header.viewMode.artists",
};

const sortLabelKeyMap: Record<SortKey, string> = {
  default: "library.header.sort.default",
  title: "library.header.sort.title",
  artist: "library.header.sort.artist",
  album: "library.header.sort.album",
  recent: "library.header.sort.recent",
};

export const LibraryHeader: React.FC<LibraryHeaderProps> = (props) => {
  const {
    totalCount,
    totalTracks,
    viewMode,
    sortKey,
    search,
    searchText,
    onSearchChange,
    onSearchTextChange,
    onViewModeChange,
    onSortKeyChange,
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

  const { theme } = useKivoTheme();
  const { colors, radius, spacing } = theme;
  const { t } = useI18n();

  // 统一曲目数量
  const resolvedTotal =
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

  const viewButton = (mode: LibraryViewMode) => {
    const isActive = viewMode === mode;

    return (
      <button
        type="button"
        onClick={() => onViewModeChange(mode)}
        style={{
          paddingLeft: spacing.md,
          paddingRight: spacing.md,
          height: 32,
          borderRadius: radius.pill,
          borderWidth: 1,
          borderStyle: "solid",
          borderColor: isActive
            ? "rgba(15, 23, 42, 0.4)"
            : "rgba(15, 23, 42, 0.12)",
          backgroundColor: isActive ? "rgba(15, 23, 42, 0.08)" : "transparent",
          cursor: "pointer",
          fontSize: 13,
          color: isActive ? colors.textOnLight : colors.textMutedOnLight,
        }}
      >
        {t(viewModeLabelKeyMap[mode])}
      </button>
    );
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
          placeholder={t("library.header.searchPlaceholder")}
          value={searchValue}
          onChange={(e) => handleSearchChange(e.target.value)}
          style={{
            flex: 1,
            height: 32,
            borderRadius: radius.pill,
            borderWidth: 1,
            borderStyle: "solid",
            borderColor: "rgba(15, 23, 42, 0.08)",
            paddingLeft: spacing.md,
            paddingRight: spacing.md,
            fontSize: 13,
          }}
        />
      </div>

      {/* 中：视图切换 */}
      <div
        style={{
          flex: 2,
          minWidth: 260,
          display: "flex",
          justifyContent: "center",
          gap: spacing.sm,
        }}
      >
        {viewButton("tracks")}
        {viewButton("albums")}
        {viewButton("artists")}
      </div>

      {/* 右：排序选择 */}
      <div
        style={{
          flex: 1,
          minWidth: 200,
          display: "flex",
          justifyContent: "flex-end",
          alignItems: "center",
          gap: spacing.sm,
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
          {(Object.keys(sortLabelKeyMap) as SortKey[]).map((key) => (
            <option key={key} value={key}>
              {t(sortLabelKeyMap[key])}
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
      <div
        style={{
          display: "flex",
          gap: spacing.sm,
        }}
      >
        <KivoButton onClick={onPlayAll}>
          {t("library.header.actions.playAll")}
        </KivoButton>
        <KivoButton variant="secondary" onClick={onShufflePlay}>
          {t("library.header.actions.shufflePlay")}
        </KivoButton>
        <KivoButton variant="ghost" onClick={handleImportClick}>
          {t("library.header.actions.importLocalMusic")}
        </KivoButton>
        <KivoButton variant="ghost" onClick={handleClearClick}>
          {t("library.header.actions.clearLibrary")}
        </KivoButton>
      </div>
    </div>
  );

  const subtitle = (
    <>
      {t("library.header.subtitle.line1")}
      <br />
      {t("library.header.subtitle.line2")}
    </>
  );

  const extraText = t("library.header.extra.totalTracks").replace(
    "{count}",
    String(resolvedTotal),
  );

  const extra = (
    <span
      style={{
        fontSize: 13,
        color: colors.textMutedOnLight,
      }}
    >
      {extraText}
    </span>
  );

  return (
    <PageHeader
      title={t("library.header.title")}
      subtitle={subtitle}
      leftBottomExtra={extra}
      centerSlot={centerSlot}
      rightSlot={rightSlot}
    />
  );
};

export default LibraryHeader;
