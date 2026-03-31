import { memo } from "react";
import { Icon } from "./Icons";

export interface DockActionsProps {
  isPlaying: boolean;
  onBack: () => void;
  onForward: () => void;
  onTogglePlay: () => void;
}

export const DockActions = memo(function DockActions({ isPlaying, onBack, onForward, onTogglePlay }: DockActionsProps) {
  return (
    <div className="controls">
      <button aria-label="Shuffle" className="dock-btn dock-btn-icon" type="button"><Icon className="dock-icon dock-shuffle"><path d="M6 8h2.8c1.1 0 1.9.3 2.7 1.1l4.5 4.6" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /><path d="M16 8h2v2" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.6" /><path d="M6 16h2.8c1.1 0 1.9-.3 2.7-1.1l4.5-4.6" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /><path d="M16 16h2v-2" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.6" /></Icon></button>
      <button aria-label="Previous" className="dock-btn dock-btn-icon" onClick={onBack} type="button"><Icon className="dock-icon dock-skip"><path d="M8.2 7v10" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /><path d="m16 7.8-6.1 4.2 6.1 4.2V7.8Z" fill="currentColor" /></Icon></button>
      <button aria-label="Play" className="play-btn" onClick={onTogglePlay} type="button">{isPlaying ? <Icon className="play-icon"><path d="M8.2 7h2.8v10H8.2zm4.8 0h2.8v10H13z" fill="currentColor" /></Icon> : <Icon className="play-icon"><path d="m9 7.6 7.4 4.4L9 16.4V7.6Z" fill="currentColor" /></Icon>}</button>
      <button aria-label="Next" className="dock-btn dock-btn-icon" onClick={onForward} type="button"><Icon className="dock-icon dock-skip"><path d="M15.8 7v10" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /><path d="m8 7.8 6.1 4.2L8 16.2V7.8Z" fill="currentColor" /></Icon></button>
      <button aria-label="Repeat" className="dock-btn dock-btn-icon" type="button"><Icon className="dock-icon dock-repeat"><path d="M8 8.6h6.6l-1.8-1.8" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.6" /><path d="M16 15.4H9.4l1.8 1.8" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.6" /><path d="M16.1 8.8c1.1.7 1.7 1.7 1.7 3.2 0 1.4-.6 2.4-1.7 3.2" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /><path d="M7.9 15.2C6.8 14.5 6.2 13.4 6.2 12s.6-2.5 1.7-3.2" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /></Icon></button>
    </div>
  );
});
