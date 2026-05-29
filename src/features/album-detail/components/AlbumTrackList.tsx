import { albumDetailData } from '../albumDetailData'

export function AlbumTrackList() {
  return (
    <section className="km-track-table" aria-label="Album tracks">
      {albumDetailData.tracks.map((track, index) => (
        <article className={index === 0 ? 'playing' : ''} key={track.id}>
          <span>{track.trackNumber}</span>
          <strong>{track.title}</strong>
          <em>{track.liked ? 'Liked track' : 'Album-only track'}</em>
          <small>{track.qualityLabel}</small>
        </article>
      ))}
    </section>
  )
}
