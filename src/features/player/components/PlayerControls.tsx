import './playerControls.css'
import { t } from '../../../shared/i18n'
import { PlayerIcon } from '../playerIcons'
import { playerControlButtons } from '../playerBarControls'
import { playerData } from '../playerData'
import { formatPlayerTime } from '../playerTime'

const currentSeconds = Math.round((playerData.track.duration * playerData.state.progress) / 100)

export function PlayerControls() {
  return (
    <div className="km-player-core">
      <div className="km-player-buttons" aria-label={t('player.controls.ariaLabel')}>
        {playerControlButtons.map((control) => (
          <button
            key={control.key}
            type="button"
            aria-label={t(control.labelKey)}
            className={`km-player-control km-player-control-${control.variant}`}
          >
            <PlayerIcon name={control.icon} />
          </button>
        ))}
      </div>
      <div className="km-player-timeline" aria-label={t('player.progress.ariaLabel')}>
        <span>{formatPlayerTime(currentSeconds)}</span>
        <div className="km-player-progress">
          <span style={{ width: `${playerData.state.progress}%` }} />
        </div>
        <span>{formatPlayerTime(playerData.track.duration)}</span>
      </div>
    </div>
  )
}
