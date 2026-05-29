import { albumDetail } from './albumDetailData'
import './albumDetail.css'

export function AlbumDetailPage() {
  const { album, artist, tracks } = albumDetail

  return (
    <section className="km-album-detail">
      <div className="km-album-detail-hero">
        <div className="km-album-detail-art" style={{ background: album.artwork }} />
        <div className="km-album-detail-copy">
          <p>Collected Album</p>
          <h2>{album.title}</h2>
          <span>{artist?.name} · {album.year} · {album.trackCount} songs</span>
          <div className="km-badge-row">
            <em>{album.qualityLabel}</em>
            <em>Album rating {album.rating}/5</em>
          </div>
          <div className="km-album-actions">
            <button type="button">Play Album</button>
            <button type="button">Shuffle</button>
          </div>
        </div>
      </div>
      <div className="km-track-list">
        {tracks.map((track) => (
          <article className="km-track-row" key={track.id}>
            <span>{track.trackNumber}</span>
            <strong>{track.title}</strong>
            <em>{track.liked ? 'Liked track' : 'Album-only track'}</em>
            <small>{track.qualityLabel}</small>
          </article>
        ))}
      </div>
    </section>
  )
}
