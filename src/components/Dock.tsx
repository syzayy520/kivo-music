import { memo, type CSSProperties } from "react";
import type { Track } from "../data/library";
import { DockActions } from "./DockActions";
import { DockExtras } from "./DockExtras";
import { ProgressBar } from "./ProgressBar";

export interface DockProps {
  onSeek: (value: number) => void;
  onSeekBack: () => void;
  onSeekForward: () => void;
  onTogglePlay: () => void;
  onVolume: (value: number) => void;
  playback: { currentTime: string; dockProgress: number; isPlaying: boolean; progress: number; volume: number };
  track: Track;
}

export const Dock = memo(function Dock(props: DockProps) {
  const { onSeek, onSeekBack, onSeekForward, onTogglePlay, onVolume, playback, track } = props;
  return (
    <footer className="dock">
      <div className="dock-now">
        <div className="dock-cover" style={track.tone ? ({ "--cover-tone": track.tone } as CSSProperties) : undefined} />
        <div className="dock-info">
          <div className="dock-title">{track.title}</div>
          <div className="dock-sub">{track.artist} {"\u00B7"} {track.album}</div>
        </div>
      </div>
      <div className="dock-center">
        <DockActions isPlaying={playback.isPlaying} onBack={onSeekBack} onForward={onSeekForward} onTogglePlay={onTogglePlay} />
        <div className="timeline">
          <span>{playback.currentTime}</span>
          <ProgressBar className="bar-input timeline-input" onChange={onSeek} value={playback.dockProgress} />
          <span>{track.duration}</span>
        </div>
      </div>
      <DockExtras onVolume={onVolume} volume={playback.volume} />
    </footer>
  );
});
