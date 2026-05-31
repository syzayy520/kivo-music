import { t } from '../../../shared/i18n'
import { listenNowCopy } from '../listenNowCopy'
import { listenNowData } from '../listenNowData'

export function ListenNowHero() {
  const { heroAlbum, heroArtist } = listenNowData
  const featuredCopy = listenNowCopy.topPicks.items[0]

  return (
    <section className="km-listen-hero" id="listen-now">
      <div className={`km-listen-hero-art artwork-${heroAlbum.artworkKey}`} />
      <div className="km-listen-hero-copy">
        <p>{t(listenNowCopy.topPicks.eyebrowKey)}</p>
        <h2>{heroAlbum.title}</h2>
        <span>{heroArtist?.name} · {heroAlbum.qualityLabel}</span>
        <strong>{t(featuredCopy.descriptionKey)}</strong>
        <div className="km-listen-actions">
          <button type="button">{t('home.hero.play')}</button>
          <button type="button" className="secondary">
            {t('home.hero.viewAlbum')}
          </button>
        </div>
      </div>
    </section>
  )
}
