import { memo } from "react";
import type { Track } from "../data/library";
import { QueueItem } from "./QueueItem";

export interface QueuePanelProps {
  currentId: string;
  onSelect: (id: string) => void;
  queue: Track[];
}

export const QueuePanel = memo(function QueuePanel({ currentId, onSelect, queue }: QueuePanelProps) {
  return (
    <aside className="column">
      <section className="card queue-card">
        <div className="card-head">
          <div><h2>{"\u63a5\u4e0b\u6765"}</h2></div>
          <button className="subtle-link" type="button">{"\u5b8c\u6574\u961f\u5217"}</button>
        </div>
        <div className="queue-list">
          {queue.map((track, index) => (
            <QueueItem active={track.id === currentId} index={index} key={track.id} onSelect={onSelect} track={track} />
          ))}
        </div>
      </section>
    </aside>
  );
});
