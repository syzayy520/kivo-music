export function parseDuration(label: string) {
  const [minutes, seconds] = label.split(":").map(Number);
  return minutes * 60 + seconds;
}

export function formatDuration(seconds: number) {
  const rounded = Math.max(0, Math.round(seconds));
  return `${Math.floor(rounded / 60)}:${String(rounded % 60).padStart(2, "0")}`;
}

export function buildPlayback(duration: string, isPlaying: boolean, progress: number, volume: number) {
  const total = parseDuration(duration);
  return {
    currentTime: formatDuration((total * progress) / 100),
    dockProgress: progress,
    isPlaying,
    progress,
    volume,
  };
}
