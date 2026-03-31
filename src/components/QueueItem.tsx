import { memo } from "react";
import type { Track } from "../data/library";
import { Icon } from "./Icons";

export interface QueueItemProps {
  active: boolean;
  index: number;
  onSelect: (id: string) => void;
  track: Track;
}

export const QueueItem = memo(function QueueItem({ active, index, onSelect, track }: QueueItemProps) {
  return (
    <button className={active ? "queue-row current" : "queue-row"} onClick={() => onSelect(track.id)} type="button">
      <div className="queue-index">{active ? <Icon className="queue-play-icon"><path d="M8 6.5v11l9-5.5-9-5.5Z" fill="currentColor" /></Icon> : String(index + 1).padStart(2, "0")}</div>
      <div>
        <span className="title-ellipsis">{track.title}</span>
        <span className="sub-ellipsis">{active ? `${track.artist} · ${track.album}` : track.artist}</span>
      </div>
      <span className="queue-time">{track.duration}</span>
    </button>
  );
});
