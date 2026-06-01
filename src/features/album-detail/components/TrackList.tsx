import { t } from '../../../shared/i18n'
import { albumDetailCopy } from '../albumDetailCopy'
import { albumDetailData } from '../albumDetailData'

export function TrackList() {
  return (
    <section className="km-track-list" aria-label={t('albumDetail.tracksAriaLabel')}>
      <p>{t(albumDetailCopy.tracksLabel)}</p>
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
