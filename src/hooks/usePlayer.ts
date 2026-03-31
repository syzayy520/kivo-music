import { useCallback, useMemo, useRef, useState } from "react";
import { nextAudio, pauseAudio, prevAudio, seekAudio, setAudioVolume, startAudio, stopAudio, type AudioStatus } from "../data/audioApi";
import { createDroppedTrack, createDroppedTrackFromName } from "../data/droppedTrack";
import { queue } from "../data/library";
import { buildPlayback, formatDuration, parseDuration } from "../data/playbackTime";
import { buildAudioStatus } from "../data/playerActions";
import type { DroppedFilePayload } from "./useFileDrop";
import { useDroppedAudio } from "./useDroppedAudio";
import { useDroppedTrackMeta } from "./useDroppedTrackMeta";
import { usePlayerPaths } from "./usePlayerPaths";
import { usePlayerSession } from "./usePlayerSession";

export function usePlayer() {
  const [currentId, setCurrentId] = useState(queue[0].id);
  const [droppedTrack, setDroppedTrack] = useState<typeof queue[number] | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [progress, setProgress] = useState(58.23);
  const [sessionId, setSessionId] = useState<number | null>(null);
  const [volume, setVolume] = useState(62);
  const droppedPayload = useRef<DroppedFilePayload | null>(null);
  const localLock = useRef(false);
  const activeQueue = useMemo(() => (droppedTrack ? [droppedTrack, ...queue] : queue), [droppedTrack]);
  const currentIndex = droppedTrack ? 0 : Math.max(0, activeQueue.findIndex((track) => track.id === currentId));
  const currentTrack = droppedTrack ?? activeQueue[currentIndex] ?? activeQueue[0];
  const { queuePaths, resolvePath } = usePlayerPaths(activeQueue);
  const playback = useMemo(() => buildPlayback(currentTrack.duration, isPlaying, progress, volume), [currentTrack.duration, isPlaying, progress, volume]);
  const droppedAudio = useDroppedAudio({ onDuration: (seconds) => setDroppedTrack((value) => (value ? { ...value, duration: formatDuration(seconds) } : value)), onEnded: () => (setIsPlaying(false), setProgress(0)), onError: () => setIsPlaying(false), onPlayState: setIsPlaying, onTime: (_, nextProgress) => setProgress(nextProgress) });
  const applyStatus = useCallback((status: AudioStatus, index: number) => {
    if (localLock.current || droppedTrack) return;
    const track = activeQueue[index] ?? currentTrack;
    const duration = status.duration ?? parseDuration(track.duration);
    setCurrentId(track.id); setSessionId(status.ended || status.state === "stopped" || status.state === "error" ? null : status.session_id); setIsPlaying(status.state === "playing"); setProgress(duration > 0 ? (status.pos / duration) * 100 : 0); setVolume(Math.round(status.volume * 100));
  }, [activeQueue, currentTrack, droppedTrack]);
  const { startDroppedPath, syncDroppedTrack } = useDroppedTrackMeta(setDroppedTrack, applyStatus, volume, localLock);
  usePlayerSession({ applyStatus, currentIndex, droppedAudioPath: droppedAudio.activePath, localMode: Boolean(droppedTrack), queuePaths, sessionId });

  const stopBackend = useCallback(() => {
    if (!sessionId) return;
    void pauseAudio(sessionId, true).catch(() => {});
    void stopAudio(sessionId).catch(() => {});
    setSessionId(null);
  }, [sessionId]);

  const playDropped = useCallback((payload: DroppedFilePayload, track = droppedTrack) => {
    if (!track) return;
    void droppedAudio.playFile(payload, volume).then((played) => {
      if (!played && payload.path) startDroppedPath(payload.path, track);
    });
  }, [droppedAudio, droppedTrack, startDroppedPath, volume]);

  const startTrack = useCallback(async (index: number) => {
    const track = activeQueue[index]; if (!track) return;
    if (droppedTrack?.id === track.id && droppedPayload.current) return void playDropped(droppedPayload.current, track);
    localLock.current = false; droppedPayload.current = null; setDroppedTrack(null); stopBackend(); if (droppedAudio.activePath) droppedAudio.stop();
    const path = queuePaths[index] ?? await resolvePath(track.id); if (!path) return;
    const nextSession = await startAudio(path); if (nextSession) applyStatus(buildAudioStatus(track, nextSession, volume), index);
  }, [activeQueue, applyStatus, droppedAudio, droppedTrack?.id, playDropped, queuePaths, resolvePath, stopBackend, volume]);

  const playDroppedFile = useCallback((payload: DroppedFilePayload) => {
    const track = payload.file ? createDroppedTrackFromName(payload.file.name) : payload.path ? createDroppedTrack(payload.path) : null; if (!track) return;
    localLock.current = true; droppedPayload.current = payload; stopBackend(); if (droppedAudio.activePath) droppedAudio.stop();
    setDroppedTrack(track); setCurrentId(track.id); setProgress(0); setIsPlaying(false);
    if (payload.path) syncDroppedTrack(payload.path, track.id);
    if (payload.file || payload.path) playDropped(payload, track);
  }, [droppedAudio, playDropped, stopBackend, syncDroppedTrack]);

  const resolveDroppedPath = useCallback((path: string) => {
    if (!droppedTrack) return;
    const payload = { ...droppedPayload.current, path };
    droppedPayload.current = payload; syncDroppedTrack(path, droppedTrack.id);
  }, [droppedTrack, syncDroppedTrack]);

  const moveTrack = useCallback((delta: -1 | 1) => {
    const nextIndex = Math.min(activeQueue.length - 1, Math.max(0, currentIndex + delta));
    if (nextIndex === currentIndex) return;
    if (!sessionId || droppedTrack) return void startTrack(nextIndex);
    void (delta > 0 ? nextAudio() : prevAudio()).then((status) => applyStatus(status, nextIndex)).catch(() => setSessionId(null));
  }, [activeQueue.length, applyStatus, currentIndex, droppedTrack, sessionId, startTrack]);

  return { queue: activeQueue, currentId: currentTrack.id, currentIndex, currentTrack, playback, playDroppedFile, resolveDroppedPath, setProgress: (value: number) => { setProgress(value); if (droppedTrack) return void droppedAudio.seek(value); if (sessionId) void seekAudio((parseDuration(currentTrack.duration) * value) / 100, sessionId).catch(() => setSessionId(null)); }, setVolume: (value: number) => { setVolume(value); if (droppedTrack) return void droppedAudio.setVolume(value); if (sessionId) void setAudioVolume(sessionId, value / 100).catch(() => setSessionId(null)); }, togglePlay: () => { if (droppedTrack) return droppedAudio.activePath ? void droppedAudio.togglePlay() : void startTrack(0); if (!sessionId) return void startTrack(currentIndex); void pauseAudio(sessionId, isPlaying).then(() => setIsPlaying((value) => !value)).catch(() => setSessionId(null)); }, selectTrack: (id: string) => void startTrack(activeQueue.findIndex((track) => track.id === id)), seekBack: () => moveTrack(-1), seekForward: () => moveTrack(1) };
}
