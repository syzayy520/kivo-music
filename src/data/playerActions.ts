import type { AudioStatus } from "./audioApi";
import type { Track } from "./library";
import { parseDuration } from "./playbackTime";

export function buildAudioStatus(track: Track, sessionId: number, volume: number): AudioStatus {
  return {
    duration: parseDuration(track.duration),
    ended: false,
    pos: 0,
    session_id: sessionId,
    state: "playing",
    volume: volume / 100,
  };
}
