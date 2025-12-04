// src/hooks/useNowPlayingCover.ts
import { useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { KivoTrackLike } from "../persistence/CoverCache";
import {
  resolveCoverPathForTrack,
  setCoverForTrack,
} from "../persistence/CoverCache";
import { log } from "../utils/log";

export interface UseNowPlayingCoverResult {
  resolvedCoverPath: string | null;
  coverSrc: string | null;
  isLoading: boolean;
  hasError: boolean;
  pickCover: () => Promise<void>;
}

/**
 * v3 版封面 Hook：
 * 1. 读封面：
 *    - 优先 track 自带 coverPath / cover / filePath / path
 *    - 否则走 CoverCache（covers.json + folder-covers.json）
 * 2. 选封面（pickCover）：
 *    - 打开文件选择器选一张图片
 *    - 调 setCoverForTrack 写入封面缓存目录 + 索引
 *    - 立即更新本地 state（resolvedCoverPath + coverSrc），UI 立刻刷新
 */
export function useNowPlayingCover(
  track: any,
  _playlist: any[],
  _currentIndex: number,
): UseNowPlayingCoverResult {
  const [resolvedCoverPath, setResolvedCoverPath] = useState<string | null>(
    null,
  );
  const [coverSrc, setCoverSrc] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [hasError, setHasError] = useState(false);

  // 稳定 key，避免 track 引用变化导致死循环
  const trackKey = useMemo(() => {
    if (!track) return "";
    if (track.trackId != null) return String(track.trackId);
    if (track.id != null) return String(track.id);
    if (typeof track.filePath === "string") return `path:${track.filePath}`;
    if (typeof track.path === "string") return `path:${track.path}`;
    return JSON.stringify({
      title: track?.title,
      name: track?.name,
      filePath: track?.filePath,
      path: track?.path,
    });
  }, [track]);

  // 统一的“读取当前封面路径”逻辑
  useEffect(() => {
    let cancelled = false;

    async function run() {
      setIsLoading(true);
      setHasError(false);

      try {
        if (!track) {
          if (!cancelled) {
            setResolvedCoverPath(null);
            setCoverSrc(null);
          }
          return;
        }

        // 1. 先尝试 track 自带的封面路径
        let path: string | null =
          (track.coverPath as string | undefined) ??
          (track.cover as string | undefined) ??
          (track.filePath as string | undefined) ??
          (track.path as string | undefined) ??
          null;

        // 2. 如果没有，再走 CoverCache / 文件夹自动封面
        if (!path) {
          try {
            path = await resolveCoverPathForTrack(
              track as KivoTrackLike,
            );
          } catch (error) {
            log.error(
              "NowPlayingCover",
              "resolveCoverPathForTrack 失败",
              { error },
            );
          }
        }

        if (!path) {
          if (!cancelled) {
            setResolvedCoverPath(null);
            setCoverSrc(null);
          }
          return;
        }

        const assetUrl = convertFileSrc(path);

        if (!cancelled) {
          setResolvedCoverPath(path);
          setCoverSrc(assetUrl);
        }
      } catch (error) {
        log.error("NowPlayingCover", "解析封面失败", { error });
        if (!cancelled) {
          setHasError(true);
          setResolvedCoverPath(null);
          setCoverSrc(null);
        }
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    run();

    return () => {
      cancelled = true;
    };
  }, [trackKey, track]);

  /**
   * 选封面流程：
   * 1. 没有当前曲目就直接返回
   * 2. 用 plugin-dialog 打开文件选择器（只让选图片）
   * 3. 调 setCoverForTrack(track, sourcePath) 写入缓存目录
   * 4. 用 convertFileSrc 转成可用的 URL，立刻更新本地 state
   */
  const pickCover = async () => {
    if (!track) {
      log.debug(
        "NowPlayingCover",
        "pickCover 被调用时没有当前曲目，忽略。",
      );
      return;
    }

    setIsLoading(true);
    setHasError(false);

    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: "图片文件",
            extensions: ["png", "jpg", "jpeg", "webp", "bmp"],
          },
        ],
      });

      if (!selected) {
        // 用户取消选择，直接返回即可
        log.debug("NowPlayingCover", "pickCover 用户取消选择");
        return;
      }

      if (Array.isArray(selected)) {
        log.warn(
          "NowPlayingCover",
          "pickCover 收到多个文件，仅取第一个。",
        );
      }

      const sourcePath = Array.isArray(selected)
        ? String(selected[0])
        : String(selected);

      const cachedPath = await setCoverForTrack(
        track as KivoTrackLike,
        sourcePath,
      );

      const assetUrl = convertFileSrc(cachedPath);

      setResolvedCoverPath(cachedPath);
      setCoverSrc(assetUrl);
      setHasError(false);

      log.info("NowPlayingCover", "封面设置成功", {
        sourcePath,
        cachedPath,
      });
    } catch (error) {
      log.error("NowPlayingCover", "pickCover 失败", { error });
      setHasError(true);
    } finally {
      setIsLoading(false);
    }
  };

  return {
    resolvedCoverPath,
    coverSrc,
    isLoading,
    hasError,
    pickCover,
  };
}
