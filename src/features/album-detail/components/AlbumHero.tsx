import { t } from '../../../shared/i18n'
import { albumDetailCopy } from '../albumDetailCopy'
import { albumDetailData } from '../albumDetailData'

export function AlbumHero() {
  const { album, artist } = albumDetailData

  return (
    <section className="km-album-hero">
      <div className={`km-album-hero-art artwork-${album.artworkKey}`} />
      <div className="km-album-hero-copy">
        <p>{t(albumDetailCopy.eyebrow)}</p>
        <h2>{album.title}</h2>
        <span>{t('albumDetail.trackCount', { artist: artist?.name ?? '', year: album.year ?? 0, count: album.trackCount })}</span>
        <div className="km-album-badges">
          <em>{album.qualityLabel}</em>
          <em>{t('albumDetail.rating', { rating: album.rating ?? 0 })}</em>
        </div>
        <div className="km-album-actions">
          <button type="button">{t(albumDetailCopy.playLabel)}</button>
          <button type="button" className="secondary">{t(albumDetailCopy.shuffleLabel)}</button>
          <button type="button" className="secondary">{t(albumDetailCopy.favoriteLabel)}</button>
        </div>
      </div>
    </section>
  )
}
