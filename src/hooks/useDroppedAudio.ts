import { useCallback, useRef, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { createAudioBlobUrl } from "../data/audioBlob";
import type { DroppedFilePayload } from "./useFileDrop";

export interface DroppedAudioHandlers {
  onDuration: (seconds: number) => void;
  onEnded: () => void;
  onError: () => void;
  onPlayState: (playing: boolean) => void;
  onTime: (seconds: number, progress: number) => void;
}

function bindAudio(audio: HTMLAudioElement, handlers: DroppedAudioHandlers, clear: () => void) {
  audio.onloadedmetadata = () => handlers.onDuration(audio.duration || 0);
  audio.ontimeupdate = () => handlers.onTime(audio.currentTime, audio.duration > 0 ? (audio.currentTime / audio.duration) * 100 : 0);
  audio.onplay = () => handlers.onPlayState(true);
  audio.onpause = () => handlers.onPlayState(false);
  audio.onended = () => (handlers.onTime(0, 0), handlers.onEnded());
  audio.onerror = () => (clear(), handlers.onError());
}

const isAbort = (error: unknown) => error instanceof DOMException && error.name === "AbortError";

export function useDroppedAudio(handlers: DroppedAudioHandlers) {
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const objectUrlRef = useRef<string | null>(null);
  const [activePath, setActivePath] = useState<string | null>(null);
  const clear = useCallback(() => {
    if (objectUrlRef.current) URL.revokeObjectURL(objectUrlRef.current);
    objectUrlRef.current = null;
    setActivePath(null);
  }, []);
  const stop = useCallback(() => {
    if (!audioRef.current) return;
    audioRef.current.pause();
    audioRef.current.src = "";
    audioRef.current = null;
    clear();
  }, [clear]);
  const playFile = useCallback(async ({ file, path }: DroppedFilePayload, volume: number) => {
    stop();
    const audio = new Audio();
    audio.volume = volume / 100;
    bindAudio(audio, handlers, clear);
    audioRef.current = audio;
    setActivePath(path ?? file?.name ?? null);
    try {
      objectUrlRef.current = file ? URL.createObjectURL(file) : path ? await createAudioBlobUrl(path) : null;
      audio.src = file ? objectUrlRef.current ?? "" : objectUrlRef.current ?? (path ? convertFileSrc(path) : "");
      await audio.play();
      return true;
    } catch (error) {
      if (isAbort(error)) return false;
      try {
        if (!path) throw new Error("no-path");
        objectUrlRef.current = await createAudioBlobUrl(path);
        audio.src = objectUrlRef.current ?? convertFileSrc(path);
        await audio.play();
        return true;
      } catch (fallbackError) {
        if (isAbort(fallbackError)) return false;
        return clear(), handlers.onError(), false;
      }
    }
  }, [clear, handlers, stop]);
  return { activePath, playFile, seek: (progress: number) => { if (audioRef.current?.duration) audioRef.current.currentTime = (audioRef.current.duration * progress) / 100; }, setVolume: (volume: number) => { if (audioRef.current) audioRef.current.volume = volume / 100; }, stop, togglePlay: () => { if (!audioRef.current) return; if (audioRef.current.paused) { if (audioRef.current.duration && audioRef.current.currentTime >= audioRef.current.duration - 0.05) audioRef.current.currentTime = 0; void audioRef.current.play().catch(() => {}); } else audioRef.current.pause(); } };
}
