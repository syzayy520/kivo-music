import './playerTrackInfo.css'
import { t } from '../../../shared/i18n'
import { playerData } from '../playerData'

export function PlayerTrackInfo() {
  return (
    <div className="km-player-track">
      <div className="km-player-art" style={playerData.artworkStyle} aria-hidden="true" />
      <section className="km-player-copy">
        <strong>{playerData.track.title}</strong>
        <span>{playerData.artist?.name ?? t('common.unknownArtist')}</span>
      </section>
      <div className="km-player-track-actions" aria-label={t('player.trackActions.ariaLabel')}>
        <button type="button" aria-label={t('player.favoriteTrack.ariaLabel')} className="km-player-action">
          <span aria-hidden="true">♡</span>
        </button>
        <button type="button" aria-label={t('player.moreTrackActions.ariaLabel')} className="km-player-action">
          <span aria-hidden="true">…</span>
        </button>
      </div>
    </div>
  )
}
