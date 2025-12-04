// src/hooks/useNowPlayingCover.ts
import { useCallback, useEffect, useMemo, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { KivoTrackLike } from "../persistence/CoverCache";
import {
  resolveCoverPathForTrack,
  setCoverForTrack,
} from "../persistence/CoverCache";
import { log } from "../utils/log";

export interface UseNowPlayingCoverResult {
  /** 实际解析出来的封面文件绝对路径（缓存或原始文件） */
  resolvedCoverPath: string | null;
  /** 可以直接给 <img src=...> 用的地址（通过 convertFileSrc 包装过） */
  coverSrc: string | null;
  /** 是否正在读取 / 写入封面 */
  isLoading: boolean;
  /** 本次操作是否出现错误（不会阻断播放） */
  hasError: boolean;
  /** 让用户从本地选择一张图片作为当前曲目的封面 */
  pickCover: () => Promise<void>;
}

/**
 * 负责“正在播放曲目”的封面解析 + 手动选择封面。
 *
 * 只依赖传入的 track / playlist / currentIndex，不直接操作全局 store。
 */
export function useNowPlayingCover(
  track: any | null,
  playlist: any[],
  currentIndex: number,
): UseNowPlayingCoverResult {
  const [resolvedCoverPath, setResolvedCoverPath] = useState<string | null>(
    null,
  );
  const [coverSrc, setCoverSrc] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [hasError, setHasError] = useState(false);

  // 如果上层传进来的 track 为空，就尝试用 playlist[currentIndex]
  const effectiveTrack = useMemo(() => {
    if (track) return track;
    if (
      Array.isArray(playlist) &&
      playlist.length > 0 &&
      currentIndex >= 0 &&
      currentIndex < playlist.length
    ) {
      return playlist[currentIndex];
    }
    return null;
  }, [track, playlist, currentIndex]);

  // 把任何 track 映射成 KivoTrackLike，大多数字段只在 CoverCache 内部用到
  const asKivoTrack = useCallback(
    (value: any | null): KivoTrackLike | null => {
      if (!value) return null;
      const anyTrack = value as any;

      const candidate: KivoTrackLike = {
        id: anyTrack.id ?? anyTrack.trackId ?? anyTrack._id,
        trackId: anyTrack.trackId ?? anyTrack.id,
        filePath:
          anyTrack.filePath ??
          anyTrack.path ??
          anyTrack.location ??
          anyTrack.fullPath,
        path: anyTrack.path ?? anyTrack.filePath,
        location: anyTrack.location,
        coverPath: anyTrack.coverPath ?? anyTrack.cover,
        cover: anyTrack.cover,
      };

      return candidate;
    },
    [],
  );

  // 当“当前曲目”变化时，重新解析封面
  useEffect(() => {
    let cancelled = false;

    async function loadCover() {
      if (!effectiveTrack) {
        setResolvedCoverPath(null);
        setCoverSrc(null);
        setHasError(false);
        return;
      }

      setIsLoading(true);
      setHasError(false);

      try {
        const kivoTrack = asKivoTrack(effectiveTrack);
        if (!kivoTrack) {
          setResolvedCoverPath(null);
          setCoverSrc(null);
          return;
        }

        const path = await resolveCoverPathForTrack(kivoTrack);
        if (cancelled) return;

        if (path) {
          setResolvedCoverPath(path);
          setCoverSrc(convertFileSrc(path));
        } else {
          setResolvedCoverPath(null);
          setCoverSrc(null);
        }
      } catch (error) {
        if (cancelled) return;
        log.error("NowPlayingCover", "加载封面失败", { error });
        setResolvedCoverPath(null);
        setCoverSrc(null);
        setHasError(true);
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    void loadCover();

    return () => {
      cancelled = true;
    };
  }, [effectiveTrack, asKivoTrack]);

  /**
   * 让用户手动选择一张图片作为封面：
   * - 只对当前曲目生效；
   * - 会写入封面缓存，并更新 hook 内部状态。
   */
  const pickCover = useCallback(async () => {
    const targetTrack = asKivoTrack(effectiveTrack);
    if (!targetTrack) {
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
        directory: false,
        title: "选择封面图片",
        filters: [
          {
            name: "图片文件",
            extensions: ["png", "jpg", "jpeg", "webp", "bmp"],
          },
        ],
      });

      if (!selected) {
        // 用户取消选择
        return;
      }

      const sourcePath = String(selected);

      const cachedPath = await setCoverForTrack(targetTrack, sourcePath);

      if (!cachedPath) {
        setHasError(true);
        return;
      }

      const assetUrl = convertFileSrc(cachedPath);

      setResolvedCoverPath(cachedPath);
      setCoverSrc(assetUrl);
      setHasError(false);
    } catch (error) {
      log.error("NowPlayingCover", "pickCover 失败", { error });
      setHasError(true);
    } finally {
      setIsLoading(false);
    }
  }, [effectiveTrack, asKivoTrack]);

  return {
    resolvedCoverPath,
    coverSrc,
    isLoading,
    hasError,
    pickCover,
  };
}
