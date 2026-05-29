import { albums } from '../../../data/mock/albums'
import { artists } from '../../../data/mock/artists'
import { listenNowCopy } from '../listenNowCopy'

function artistName(artistId: string) {
  return artists.find((artist) => artist.id === artistId)?.name ?? 'Unknown Artist'
}

export function ListenNowHero() {
  const heroAlbum = albums[0]

  return (
    <section className="km-hero" id="listen-now">
      <div className={`km-hero-art ${heroAlbum.artworkClass}`} aria-label={`${heroAlbum.title} artwork`} />
      <div className="km-hero-copy">
        <p>{listenNowCopy.heroEyebrow}</p>
        <h2>{listenNowCopy.heroTitle}</h2>
        <span>{artistName(heroAlbum.artistId)} · {heroAlbum.qualityLabel}</span>
        <strong>{listenNowCopy.heroDescription}</strong>
        <div className="km-actions">
          <button type="button">Play</button>
          <button type="button" className="secondary">View Album</button>
        </div>
      </div>
    </section>
  )
}
