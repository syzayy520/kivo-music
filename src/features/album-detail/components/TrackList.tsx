import { albumDetailCopy } from '../albumDetailCopy'
import { albumDetailData } from '../albumDetailData'

export function TrackList() {
  return (
    <section className="km-track-list" aria-label="Album tracks">
      <p>{albumDetailCopy.tracksLabel}</p>
      {albumDetailData.tracks.map((track) => (
        <article key={track.id}>
          <span>{track.trackNumber}</span>
          <strong>{track.title}</strong>
          <em>{track.qualityLabel}</em>
        </article>
      ))}
    </section>
  )
}
