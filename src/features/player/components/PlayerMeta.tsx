import './playerMeta.css'
import { t } from '../../../shared/i18n'
import { PlayerIcon } from '../playerIcons'
import { playerData } from '../playerData'

export function PlayerMeta() {
  return (
    <div className="km-player-meta" aria-label={t('player.status.ariaLabel')}>
      <span className="km-player-quality">{playerData.qualityShortLabel}</span>
      <div className="km-player-volume" aria-label={t('player.volume.ariaLabel')}>
        <PlayerIcon name="volume" />
        <div>
          <span style={{ width: `${playerData.state.volume}%` }} />
        </div>
      </div>
      <button type="button" className="km-player-queue">
        {t('player.queue')}
      </button>
    </div>
  )
}
