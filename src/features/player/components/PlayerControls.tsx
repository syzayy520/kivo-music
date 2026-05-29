import { playerData } from '../playerData'

export function PlayerControls() {
  return (
    <div className="km-player-core">
      <div className="km-player-buttons">
        <button type="button">Prev</button>
        <button type="button" className="play">Pause</button>
        <button type="button">Next</button>
      </div>
      <div className="km-progress">
        <span style={{ width: `${playerData.state.progress}%` }} />
      </div>
    </div>
  )
}
