// src/persistence/CoverCache.ts

export type {
  KivoTrackLike,
  CoverCacheStats,
} from "./cover-cache.api";

export {
  getCoverCacheDir,
  ensureCoverCacheReady,
  getCachedCoverPath,
  setCoverForTrack,
  resolveCoverPathForTrack,
  getCoverCacheStats,
  clearCoverCache,
  cleanupBrokenCoverEntries,
  migrateCoverCache,
  formatBytes,
} from "./cover-cache.api";

export type {
  CoverRecord,
  CoverIndex,
  FolderCoverRecord,
  FolderCoverIndex,
  BrokenCoverCleanupResult,
} from "./cover-cache.index";
