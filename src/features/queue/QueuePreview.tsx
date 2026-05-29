import { tracks } from '../../data/mock/tracks'
import { playerState } from '../player/playerState'

export function QueuePreview() {
  const queuedTracks = playerState.queue
    .map((trackId) => tracks.find((track) => track.id === trackId))
    .filter((track) => track !== undefined)

  return (
    <section className="km-queue" aria-label="Queue preview">
      <div className="km-queue-head">
        <p>Up Next</p>
        <button type="button">Edit Queue</button>
      </div>
      {queuedTracks.map((track) => (
        <article key={track.id}>
          <strong>{track.title}</strong>
          <span>{track.qualityLabel}</span>
        </article>
      ))}
    </section>
  )
}
