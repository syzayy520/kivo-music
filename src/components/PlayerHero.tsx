import { memo, type CSSProperties } from "react";
import type { Track } from "../data/library";
import { ProgressBar } from "./ProgressBar";

export interface PlayerHeroProps {
  onSeek: (value: number) => void;
  onTogglePlay: () => void;
  playback: { currentTime: string; isPlaying: boolean; progress: number };
  queueIndex: number;
  track: Track;
}

export const PlayerHero = memo(function PlayerHero({ onSeek, onTogglePlay, playback, queueIndex, track }: PlayerHeroProps) {
  const stateLabel = playback.isPlaying ? "播放到" : "暂停在";
  return (
    <section className="hero">
      <div aria-hidden="true" className="cover" style={track.tone ? ({ "--cover-tone": track.tone } as CSSProperties) : undefined} />
      <div className="hero-body">
        <div>
          <h1 className="hero-title">{track.title}</h1>
          <p className="hero-sub">{track.artist} <span>· {track.album}</span></p>
        </div>
        <div className="hero-meta">
          <span>{`当前队列第 ${queueIndex + 1} 首`}</span>
          <span>{`${stateLabel} ${playback.currentTime}`}</span>
        </div>
        <div className="progress">
          <div className="progress-meta"><span>{playback.currentTime}</span><span>{track.duration}</span></div>
          <ProgressBar className="bar-input progress-input" onChange={onSeek} value={playback.progress} />
        </div>
      </div>
      <div className="hero-side">
        <button className="primary-btn" onClick={onTogglePlay} type="button">{playback.isPlaying ? "暂停播放" : "继续播放"}</button>
        <div className="hero-secondary-actions">
          <button className="hero-link" type="button">查看专辑</button>
          <button className="hero-link" type="button">加入收藏</button>
        </div>
      </div>
    </section>
  );
});
