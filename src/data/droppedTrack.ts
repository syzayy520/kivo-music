import type { Track } from "./library";
import { formatDuration } from "./playbackTime";

const AUDIO_EXTENSIONS = [".mp3", ".flac", ".wav", ".m4a", ".aac", ".ogg", ".opus"];
const TONES = [
  "linear-gradient(135deg,#e6d4c3 0%,#f4e7da 42%,#ad8565 100%)",
  "linear-gradient(135deg,#deccb9 0%,#ead7c9 42%,#8f6a50 100%)",
  "linear-gradient(135deg,#ead7c5 0%,#f2e6d8 42%,#9a7358 100%)",
];

export interface DroppedTrackMetadata {
  album?: string | null;
  artist?: string | null;
  duration_seconds?: number | null;
  title?: string | null;
}

const cleanExtension = (value: string) => value.replace(/\.(mp3|flac|wav|m4a|aac|ogg|opus)$/i, "");
const cleanPrefix = (value: string) => value.replace(/^\d{1,2}[.\-_\s]+/, "");
const cleanLabel = (value: string) => cleanPrefix(cleanExtension(value)).trim();

function splitArtistAndTitle(value: string) {
  const parts = value.split(/\s*[-_~]+\s*|\s*[·•]\s*/);
  if (parts.length < 2) return null;
  const [artist, ...rest] = parts;
  const title = rest.join(" - ").trim();
  return artist.trim() && title ? { artist: artist.trim(), title } : null;
}

function albumFromPath(path?: string) {
  if (!path) return null;
  const parts = path.split(/[/\\]/).filter(Boolean);
  return parts.length > 1 ? cleanLabel(parts[parts.length - 2]) : null;
}

function toneFor(seed: string) {
  const hash = Array.from(seed).reduce((value, char) => value + char.charCodeAt(0), 0);
  return TONES[hash % TONES.length];
}

function buildDroppedTrack(title: string, path?: string, metadata?: DroppedTrackMetadata): Track {
  const baseLabel = cleanLabel(title);
  const fallback = splitArtistAndTitle(baseLabel);
  const cleanTitle = cleanLabel(metadata?.title?.trim() || fallback?.title || title);
  return {
    album: metadata?.album?.trim() || albumFromPath(path) || "Music",
    artist: metadata?.artist?.trim() || fallback?.artist || "本地文件",
    duration: metadata?.duration_seconds ? formatDuration(metadata.duration_seconds) : "0:00",
    id: `drop:${baseLabel}`,
    path,
    title: cleanTitle,
    tone: toneFor(path ?? cleanTitle),
  };
}

export function isAudioPath(path: string) {
  const lower = path.toLowerCase();
  return AUDIO_EXTENSIONS.some((extension) => lower.endsWith(extension));
}

export function createDroppedTrack(path: string, metadata?: DroppedTrackMetadata) {
  const segment = path.split(/[/\\]/).pop() ?? path;
  return buildDroppedTrack(segment, path, metadata);
}

export function createDroppedTrackFromName(name: string, metadata?: DroppedTrackMetadata) {
  return buildDroppedTrack(name, undefined, metadata);
}
