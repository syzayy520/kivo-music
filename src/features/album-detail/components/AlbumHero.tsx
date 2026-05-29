import { albumDetailData } from '../albumDetailData'

export function AlbumHero() {
  const { album, artist } = albumDetailData

  return (
    <section className="km-album-detail">
      <div className={`km-album-art ${album.artworkClass}`} />
      <div className="km-album-copy">
        <p>Collected Album</p>
        <h2>{album.title}</h2>
        <span>{artist?.name} · {album.year} · {album.trackCount} songs · 42 min</span>
        <div className="km-badges">
          <em>Lossless</em>
          <em>{album.format}</em>
          <em>{album.qualityLabel}</em>
          <em>Album rating {album.rating}/5</em>
        </div>
        <div className="km-actions">
          <button type="button">Play Album</button>
          <button type="button" className="secondary">Shuffle</button>
          <button type="button" className="secondary">Favorite Album</button>
        </div>
      </div>
    </section>
  )
}
