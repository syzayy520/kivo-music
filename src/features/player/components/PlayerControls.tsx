import { playerControlButtons } from '../playerBarControls'
import { playerData } from '../playerData'
import { formatPlayerTime } from '../playerTime'

const currentSeconds = Math.round((playerData.track.duration * playerData.state.progress) / 100)

export function PlayerControls() {
  return (
    <div className="km-player-core">
      <div className="km-player-buttons" aria-label="Playback controls">
        {playerControlButtons.map((control) => (
          <button
            key={control.key}
            type="button"
            aria-label={control.label}
            className={`km-player-control km-player-control-${control.variant}`}
          >
            {control.symbol}
          </button>
        ))}
      </div>
      <div className="km-player-timeline" aria-label="Playback progress">
        <span>{formatPlayerTime(currentSeconds)}</span>
        <div className="km-player-progress">
          <span style={{ width: `${playerData.state.progress}%` }} />
        </div>
        <span>{formatPlayerTime(playerData.track.duration)}</span>
      </div>
    </div>
  )
}
