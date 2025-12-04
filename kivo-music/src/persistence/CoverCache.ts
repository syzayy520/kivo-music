// src/persistence/CoverCache.ts
// 对外统一入口：其它模块仍然从 "./CoverCache" 导入即可。

export type {
  KivoTrackLike,
  CoverRecord,
  CoverIndex,
  FolderCoverRecord,
  FolderCoverIndex,
  CoverCacheStats,
  BrokenCoverCleanupResult,
} from "./cover-cache.index";

export {
  ensureCoverCacheReady,
  getCoverCacheDir,
  setCoverForTrack,
  getCachedCoverPath,
  getFolderCoverForTrack,
  resolveCoverPathForTrack,
  getCoverCacheStats,
  clearCoverCache,
  cleanupBrokenCoverEntries,
  migrateCoverCache,
} from "./cover-cache.api";
