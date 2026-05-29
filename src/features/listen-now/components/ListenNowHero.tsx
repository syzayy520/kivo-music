import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

export function ListenNowHero() {
  const { heroAlbum, heroArtist } = listenNowData

  return (
    <section className="km-listen-hero" id="listen-now">
      <div className={`km-listen-hero-art artwork-${heroAlbum.artworkKey}`} />
      <div className="km-listen-hero-copy">
        <p>{listenNowCopy.heroEyebrow}</p>
        <h2>{listenNowCopy.heroTitle}</h2>
        <span>{heroArtist?.name} · {heroAlbum.qualityLabel}</span>
        <strong>{listenNowCopy.heroDescription}</strong>
        <div className="km-listen-actions">
          <button type="button">Play</button>
          <button type="button" className="secondary">View Album</button>
        </div>
      </div>
    </section>
  )
}
