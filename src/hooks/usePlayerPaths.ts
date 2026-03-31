import { useCallback, useEffect, useRef, useState } from "react";
import type { Track } from "../data/library";
import { searchLibraryPrefix } from "../data/libraryApi";

export function usePlayerPaths(tracks: Track[]) {
  const cache = useRef<Record<string, string>>({});
  const [queuePaths, setQueuePaths] = useState<Array<string | null>>([]);

  const resolvePath = useCallback(async (id: string) => {
    if (cache.current[id]) return cache.current[id];
    const track = tracks.find((item) => item.id === id);
    if (track?.path) {
      cache.current[id] = track.path;
      return track.path;
    }
    const hits = track ? await searchLibraryPrefix(track.title) : [];
    const path = hits[0]?.path ?? null;
    if (path) cache.current[id] = path;
    return path;
  }, [tracks]);

  useEffect(() => {
    let cancelled = false;
    void Promise.all(tracks.map((track) => resolvePath(track.id))).then((paths) => {
      if (!cancelled) setQueuePaths(paths);
    });
    return () => {
      cancelled = true;
    };
  }, [resolvePath, tracks]);

  return { queuePaths, resolvePath };
}
