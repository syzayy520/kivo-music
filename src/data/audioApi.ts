import { invoke } from "@tauri-apps/api/core";

export interface AudioStatus {
  duration: number | null;
  ended: boolean;
  pos: number;
  session_id: number;
  state: "starting" | "playing" | "paused" | "stopping" | "stopped" | "error";
  volume: number;
}

export interface QueueSnapshot {
  autoplay: boolean;
  current_index: number | null;
  current_path: string | null;
  items: string[];
}

export interface AudioTick {
  queue: QueueSnapshot;
  status: AudioStatus;
}

export interface AudioFileMetadata {
  album: string | null;
  artist: string | null;
  duration_seconds: number;
  title: string | null;
}

export async function startAudio(path: string) {
  try { return await invoke<number>("audio_service_play_start", { inputPath: path }); } catch { return null; }
}
export async function fetchAudioStatus(sessionId: number) {
  try { return await invoke<AudioStatus>("audio_service_status", { sessionId }); } catch { return null; }
}
export async function fetchAudioTick() {
  try { return await invoke<AudioTick>("audio_service_tick"); } catch { return null; }
}
export async function readAudioBytes(path: string) {
  try { return await invoke<number[]>("audio_file_read_bytes", { inputPath: path }); } catch { return null; }
}
export async function fetchAudioMetadata(path: string) {
  try { return await invoke<AudioFileMetadata>("audio_file_probe", { inputPath: path }); } catch { return null; }
}
export function pauseAudio(sessionId: number, paused: boolean) { return invoke("audio_service_pause", { paused, sessionId }); }
export function seekAudio(seconds: number, sessionId: number) { return invoke("audio_service_seek", { seconds, sessionId }); }
export function setAudioVolume(sessionId: number, volume: number) { return invoke("audio_service_set_volume", { sessionId, volume }); }
export function stopAudio(sessionId: number) { return invoke("audio_service_stop", { sessionId }); }
export function setAudioQueue(items: string[], startIndex: number) { return invoke("audio_service_queue_set", { autoplay: true, items, startIndex }); }
export function nextAudio() { return invoke<AudioStatus>("audio_service_next"); }
export function prevAudio() { return invoke<AudioStatus>("audio_service_prev"); }
