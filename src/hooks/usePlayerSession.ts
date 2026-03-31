import { useEffect } from "react";
import { fetchAudioTick, setAudioQueue, type AudioStatus } from "../data/audioApi";

export interface UsePlayerSessionOptions {
  applyStatus: (status: AudioStatus, index: number) => void;
  currentIndex: number;
  droppedAudioPath: string | null;
  localMode: boolean;
  queuePaths: Array<string | null>;
  sessionId: number | null;
}

export function usePlayerSession(options: UsePlayerSessionOptions) {
  const { applyStatus, currentIndex, droppedAudioPath, localMode, queuePaths, sessionId } = options;

  useEffect(() => {
    if (sessionId || droppedAudioPath || localMode || !queuePaths.length || !queuePaths.every(Boolean)) return;
    void setAudioQueue(queuePaths as string[], currentIndex);
  }, [currentIndex, droppedAudioPath, localMode, queuePaths, sessionId]);

  useEffect(() => {
    if (!sessionId || droppedAudioPath || localMode) return;
    const timer = setInterval(() => {
      void fetchAudioTick().then((tick) => {
        if (!tick) return;
        applyStatus(tick.status, Math.max(0, tick.queue.current_index ?? currentIndex));
      });
    }, 1000);
    return () => clearInterval(timer);
  }, [applyStatus, currentIndex, droppedAudioPath, localMode, sessionId]);
}
