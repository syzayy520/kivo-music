import './playerMeta.css'
import { PlayerIcon } from '../playerIcons'
import { playerData } from '../playerData'

export function PlayerMeta() {
  return (
    <div className="km-player-meta" aria-label="Player status">
      <span className="km-player-quality">{playerData.qualityShortLabel}</span>
      <div className="km-player-volume" aria-label="Volume">
        <PlayerIcon name="volume" />
        <div>
          <span style={{ width: `${playerData.state.volume}%` }} />
        </div>
      </div>
      <button type="button" className="km-player-queue">
        Queue
      </button>
    </div>
  )
}
