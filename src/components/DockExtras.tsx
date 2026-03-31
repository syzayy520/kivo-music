import { memo } from "react";
import { ProgressBar } from "./ProgressBar";
import { Icon } from "./Icons";

export interface DockExtrasProps {
  onVolume: (value: number) => void;
  volume: number;
}

export const DockExtras = memo(function DockExtras({ onVolume, volume }: DockExtrasProps) {
  return (
    <div className="dock-side">
      <button aria-label="Queue" className="dock-btn dock-btn-icon" type="button"><Icon className="dock-icon dock-list"><path d="M6 7h12M6 12h9M6 17h12" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.7" /></Icon></button>
      <div className="volume">
        <button aria-label="Volume" className="dock-btn dock-btn-icon" type="button"><Icon className="dock-icon dock-speaker"><path d="M6 14h2.8l3.6 3.4V6.6L8.8 10H6v4Z" fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.6" /><path d="M15.1 10a3.3 3.3 0 0 1 0 4" fill="none" stroke="currentColor" strokeLinecap="round" strokeWidth="1.6" /></Icon></button>
        <ProgressBar className="bar-input volume-input" onChange={onVolume} value={volume} />
      </div>
      <button aria-label="More" className="dock-btn dock-btn-icon" type="button"><Icon className="dock-icon dock-more"><circle cx="7.5" cy="12" r="1.15" fill="currentColor" /><circle cx="12" cy="12" r="1.15" fill="currentColor" /><circle cx="16.5" cy="12" r="1.15" fill="currentColor" /></Icon></button>
    </div>
  );
});
