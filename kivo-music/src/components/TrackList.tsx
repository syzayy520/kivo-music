import React, {
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useVirtualizer } from "@tanstack/react-virtual";
import { usePlayerStore } from "../store/player";
import { loadLibrary, saveLibrary } from "../persistence/LibraryPersistence";

type AnyTrack = any;

// 根据 id / 标题 生成渐变颜色（列表小封面和占位用）
const pickGradientForKey = (key: string | undefined): [string, string] => {
  const palettes: [string, string][] = [
    ["#6366f1", "#ec4899"],
    ["#0ea5e9", "#22c55e"],
    ["#f97316", "#facc15"],
    ["#14b8a6", "#3b82f6"],
    ["#e11d48", "#f97316"],
    ["#8b5cf6", "#ec4899"],
  ];

  const k = key && key.length > 0 ? key : "kivo-default";
  let hash = 0;
  for (let i = 0; i < k.length; i++) {
    hash = (hash * 31 + k.charCodeAt(i)) | 0;
  }
  const index = Math.abs(hash) % palettes.length;
  return palettes[index];
};

// 从文件路径推断封面图片的本地路径（不访问硬盘，只做字符串处理）
function guessCoverPathFromFilePath(filePath: string | undefined): string | null {
  if (!filePath) return null;

  const str = String(filePath);
  const parts = str.split(/[/\\]/);
  if (parts.length === 0) return null;

  const fileName = parts[parts.length - 1];
  const dir = parts.slice(0, parts.length - 1).join(
    str.includes("\\") ? "\\" : "/",
  );

  const baseName = fileName.replace(/\.[^.]+$/, ""); // 去掉扩展名

  const candidates = [
    "cover.jpg",
    "cover.png",
    "folder.jpg",
    "folder.png",
    "front.jpg",
    "front.png",
    `${baseName}.jpg`,
    `${baseName}.png`,
  ];

  const sep =
    dir.endsWith("\\") || dir.endsWith("/")
      ? ""
      : str.includes("\\")
      ? "\\"
      : "/";

  return dir ? `${dir}${sep}${candidates[0]}` : candidates[0];
}

export const TrackList: React.FC = () => {
  // 用多个 selector，避免订阅整个 state 造成没必要的重渲染
  const playlist = usePlayerStore((s: any) => s.playlist ?? s.tracks ?? []);
  const currentIndex = usePlayerStore((s: any) => s.currentIndex ?? 0);
  const setPlaylist = usePlayerStore(
    (s: any) => s.setPlaylist ?? s.setTracks ?? (() => {}),
  );
  const playTrack = usePlayerStore(
    (s: any) => s.playTrack ?? s.play ?? (() => {}),
  );

  // 记录「猜封面」失败过的歌曲，避免每次滚动都去请求不存在的 cover.jpg
  const [failedGuessMap, setFailedGuessMap] = useState<Record<string, boolean>>(
    {},
  );

  // 排序状态
  const [sortKey, setSortKey] = useState<"none" | "title" | "artist">("none");
  const [sortDir, setSortDir] = useState<"asc" | "desc">("asc");

  // 只在首次渲染时从磁盘加载一次
  const hasLoadedRef = useRef(false);

  useEffect(() => {
    if (hasLoadedRef.current) return;
    hasLoadedRef.current = true;

    (async () => {
      try {
        const tracksFromDisk = await loadLibrary();
        if (tracksFromDisk && tracksFromDisk.length > 0) {
          setPlaylist(tracksFromDisk as AnyTrack[]);
          console.info(
            "[TrackList] loaded library from disk, tracks:",
            tracksFromDisk.length,
          );
        }
      } catch (error) {
        console.error("[TrackList] loadLibrary failed:", error);
      }
    })();
  }, [setPlaylist]);

  // 导入本地音乐文件
  const handleImportClick = async () => {
    try {
      const result = await open({
        multiple: true,
        filters: [
          {
            name: "Audio",
            extensions: ["mp3", "flac", "wav", "m4a", "ogg"],
          },
        ],
      });

      if (!result) return;

      const paths = Array.isArray(result) ? result : [result];

      const newTracks: AnyTrack[] = paths.map((p, i) => {
        const path = String(p);
        const parts = path.split(/[\\/]/);
        const filename = parts[parts.length - 1] || "未知文件";
        const title = filename.replace(/\.[^.]+$/, "");

        return {
          id: `local-${Date.now()}-${i}`,
          title,
          artist: "未知艺人",
          album: undefined,
          filePath: path,
        };
      });

      const merged = [...playlist, ...newTracks];
      setPlaylist(merged as AnyTrack[]);

      try {
        await saveLibrary(merged as AnyTrack[]);
        console.info("[TrackList] saveLibrary ok, tracks:", merged.length);
      } catch (error) {
        console.error("[TrackList] saveLibrary failed:", error);
      }
    } catch (err) {
      console.error("[TrackList] 导入本地文件失败：", err);
    }
  };

  const handleRowClick = (realIndex: number) => {
    try {
      playTrack(realIndex);
    } catch (err) {
      console.error("[TrackList] playTrack 调用失败：", err);
    }
  };

  const hasTracks = playlist && playlist.length > 0;

  // --- 搜索逻辑 ---
  const [query, setQuery] = useState("");
  const normalizedQuery = query.trim().toLowerCase();

  // 小工具：切换排序
    // 切换排序字段 / 方向
  const toggleSort = (key: "title" | "artist") => {
    if (sortKey !== key) {
      // 换了新的字段：默认升序
      setSortKey(key);
      setSortDir("asc");
    } else {
      // 同一个字段：在 升序/降序 之间切换
      setSortDir((prev) => (prev === "asc" ? "desc" : "asc"));
    }
  };


  const sortArrow = (key: "title" | "artist") => {
    if (sortKey !== key) return null;
    return sortDir === "asc" ? " ↑" : " ↓";
  };

  // 只存「匹配到的原始索引」，再根据排序设置排序
  const filteredAndSortedIndices = useMemo(() => {
    if (!hasTracks) return [] as number[];

    // 1. 先做过滤
    let indices: number[] = [];

    if (!normalizedQuery) {
      indices = playlist.map((_, idx) => idx);
    } else {
      const result: number[] = [];
      for (let i = 0; i < playlist.length; i++) {
        const t = playlist[i];
        const title = String(t?.title ?? "").toLowerCase();
        const artist = String(t?.artist ?? "").toLowerCase();
        const album = String(t?.album ?? "").toLowerCase();

        if (
          title.includes(normalizedQuery) ||
          artist.includes(normalizedQuery) ||
          album.includes(normalizedQuery)
        ) {
          result.push(i);
        }
      }
      indices = result;
    }

    // 2. 再按排序设置排序（不改变原数组顺序）
    if (sortKey === "none") return indices;

    const sorted = [...indices];

    sorted.sort((aIndex, bIndex) => {
      const a = playlist[aIndex];
      const b = playlist[bIndex];

      const aVal =
        sortKey === "title"
          ? String(a?.title ?? "")
          : String(a?.artist ?? "");
      const bVal =
        sortKey === "title"
          ? String(b?.title ?? "")
          : String(b?.artist ?? "");

      const cmp = aVal.localeCompare(bVal, "zh-CN", {
        sensitivity: "base",
      });
      return sortDir === "asc" ? cmp : -cmp;
    });

    return sorted;
  }, [playlist, normalizedQuery, hasTracks, sortKey, sortDir]);

  // --- 虚拟列表相关 ---

  const parentRef = useRef<HTMLDivElement | null>(null);

  const rowVirtualizer = useVirtualizer({
    count: filteredAndSortedIndices.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 40, // 行高稍微大一点，给缩略图留空间
    overscan: 8,
  });

  return (
    <div className="p-4 flex flex-col gap-4">
      {/* 顶部：导入按钮 + 歌曲统计 + 搜索框 */}
      <div className="flex items-center gap-2 flex-wrap">
        <button
          onClick={handleImportClick}
          className="px-3 py-1.5 rounded bg-purple-600 text-white text-sm hover:bg-purple-700"
        >
          + 导入本地音乐文件
        </button>

        {hasTracks && (
          <div className="text-xs text-gray-500">
            共 {playlist.length} 首
            {normalizedQuery && <>，匹配 {filteredAndSortedIndices.length} 首</>}
          </div>
        )}

        <div className="ml-auto">
          <input
            className="border border-gray-300 rounded px-2 py-1 text-sm w-56"
            placeholder="搜索标题 / 艺人 / 专辑"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
      </div>

      {!hasTracks ? (
        <div className="text-sm text-gray-500">
          当前还没有任何歌曲，请先点击上方按钮导入本地音乐文件。
        </div>
      ) : (
        <div className="border border-gray-200 rounded overflow-hidden text-sm">
          {/* 表头固定 */}
          <div className="bg-gray-50 border-b border-gray-200 flex">
            <div className="w-12 px-3 py-2 text-right font-medium text-gray-500">
              #
            </div>
            <div className="w-16 px-3 py-2 text-left font-medium text-gray-600">
              封面
            </div>
            <div className="flex-1 px-3 py-2 text-left font-medium text-gray-600">
              <button
                type="button"
                onClick={() => toggleSort("title")}
                className="inline-flex items-center gap-1"
              >
                标题
                <span className="text-xs">{sortArrow("title")}</span>
              </button>
            </div>
            <div className="w-48 px-3 py-2 text-left font-medium text-gray-600">
              <button
                type="button"
                onClick={() => toggleSort("artist")}
                className="inline-flex items-center gap-1"
              >
                艺人
                <span className="text-xs">{sortArrow("artist")}</span>
              </button>
            </div>
          </div>

          {/* 虚拟滚动区域 */}
          <div
            ref={parentRef}
            style={{
              height: "60vh",
              overflowY: "auto",
              position: "relative",
            }}
          >
            <div
              style={{
                height: rowVirtualizer.getTotalSize(),
                width: "100%",
                position: "relative",
              }}
            >
              {rowVirtualizer.getVirtualItems().map((virtualRow) => {
                const rowIndex = virtualRow.index; // 过滤+排序后的行号
                const realIndex = filteredAndSortedIndices[rowIndex]; // 对应 playlist 里的真实索引
                const track = playlist[realIndex];
                if (!track) return null;

                const active = realIndex === currentIndex;
                const title = track?.title ?? "未知标题";
                const artist = track?.artist ?? "未知艺人";

                const [c1, c2] = pickGradientForKey(
                  track?.id ?? track?.title ?? track?.filePath,
                );

                const trackKey = String(track?.id ?? realIndex);

                // 1. 手动封面（在「正在播放」里选的 coverPath）
                const manualCoverPath: string | undefined = track?.coverPath;

                // 2. 自动猜同目录的 cover.jpg 等（只有在没有手动封面时才会用）
                const guessedCoverPath: string | null =
                  !manualCoverPath
                    ? guessCoverPathFromFilePath(track?.filePath)
                    : null;

                const hasFailedGuess = !!failedGuessMap[trackKey];

                let coverSrc: string | null = null;
                let usingGuess = false;

                if (manualCoverPath) {
                  coverSrc = convertFileSrc(String(manualCoverPath));
                } else if (guessedCoverPath && !hasFailedGuess) {
                  coverSrc = convertFileSrc(String(guessedCoverPath));
                  usingGuess = true;
                }

                return (
                  <div
                    key={track?.id ?? realIndex}
                    onClick={() => handleRowClick(realIndex)}
                    className={
                      "flex items-center cursor-pointer select-none border-b border-gray-100 " +
                      (active
                        ? "bg-purple-50"
                        : "hover:bg-gray-50 transition-colors")
                    }
                    style={{
                      position: "absolute",
                      top: 0,
                      left: 0,
                      width: "100%",
                      height: virtualRow.size,
                      transform: `translateY(${virtualRow.start}px)`,
                    }}
                  >
                    <div className="w-12 px-3 text-right text-gray-500">
                      {realIndex + 1}
                    </div>

                    {/* 小封面缩略图 */}
                    <div className="w-16 px-3 flex items-center justify-center">
                      {coverSrc ? (
                        <img
                          src={coverSrc}
                          alt=""
                          style={{
                            width: 28,
                            height: 28,
                            borderRadius: 6,
                            objectFit: "cover",
                            boxShadow: active
                              ? "0 0 0 2px rgba(79,70,229,0.5)"
                              : "none",
                          }}
                          onError={() => {
                            if (usingGuess) {
                              setFailedGuessMap((prev) => ({
                                ...prev,
                                [trackKey]: true,
                              }));
                            }
                          }}
                        />
                      ) : (
                        <div
                          style={{
                            width: 24,
                            height: 24,
                            borderRadius: 6,
                            backgroundImage: `linear-gradient(135deg, ${c1}, ${c2})`,
                            boxShadow: active
                              ? "0 0 0 2px rgba(79,70,229,0.5)"
                              : "none",
                          }}
                        />
                      )}
                    </div>

                    <div className="flex-1 px-3 truncate">
                      <span className={active ? "font-semibold" : ""}>
                        {title}
                      </span>
                    </div>
                    <div className="w-48 px-3 text-gray-500 truncate">
                      {artist}
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default TrackList;
