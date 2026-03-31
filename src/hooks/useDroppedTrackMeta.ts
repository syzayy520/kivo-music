import { useCallback, type Dispatch, type MutableRefObject, type SetStateAction } from "react";
import { fetchAudioMetadata, startAudio, type AudioStatus } from "../data/audioApi";
import { createDroppedTrack } from "../data/droppedTrack";
import type { Track } from "../data/library";
import { buildAudioStatus } from "../data/playerActions";

export function useDroppedTrackMeta(setDroppedTrack: Dispatch<SetStateAction<Track | null>>, applyStatus: (status: AudioStatus, index: number) => void, volume: number, localLock: MutableRefObject<boolean>) {
  const syncDroppedTrack = useCallback((path: string, id: string) => {
    void fetchAudioMetadata(path).then((metadata) => setDroppedTrack((value) => (value?.id === id ? { ...createDroppedTrack(path, metadata ?? undefined), id } : value)));
  }, [setDroppedTrack]);

  const startDroppedPath = useCallback((path: string, track: Track) => {
    void startAudio(path).then((nextSession) => {
      localLock.current = false;
      if (nextSession) applyStatus(buildAudioStatus(track, nextSession, volume), 0);
    }).catch(() => {
      localLock.current = false;
    });
  }, [applyStatus, localLock, volume]);

  return { startDroppedPath, syncDroppedTrack };
}
