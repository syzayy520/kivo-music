import { queueData } from './queueData'
import './queue.css'

export function QueuePreview() {
  return (
    <section className="km-queue">
      <div className="km-queue-head">
        <p>Up Next</p>
        <button type="button">Queue</button>
      </div>
      {queueData.tracks.map((track) => (
        <article key={track.id}>
          <strong>{track.title}</strong>
          <span>{track.qualityLabel}</span>
        </article>
      ))}
    </section>
  )
}
