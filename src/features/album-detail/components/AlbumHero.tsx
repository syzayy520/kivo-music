import { albumDetailCopy } from '../albumDetailCopy'
import { albumDetailData } from '../albumDetailData'

export function AlbumHero() {
  const { album, artist } = albumDetailData

  return (
    <section className="km-album-hero">
      <div className={`km-album-hero-art artwork-${album.artworkKey}`} />
      <div className="km-album-hero-copy">
        <p>{albumDetailCopy.eyebrow}</p>
        <h2>{album.title}</h2>
        <span>{artist?.name} · {album.year} · {album.trackCount} songs</span>
        <div className="km-album-badges">
          <em>{album.qualityLabel}</em>
          <em>Rating {album.rating}/5</em>
        </div>
        <div className="km-album-actions">
          <button type="button">{albumDetailCopy.playLabel}</button>
          <button type="button" className="secondary">{albumDetailCopy.shuffleLabel}</button>
          <button type="button" className="secondary">{albumDetailCopy.favoriteLabel}</button>
        </div>
      </div>
    </section>
  )
}
