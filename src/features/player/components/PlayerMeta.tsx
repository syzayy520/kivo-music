import { playerData } from '../playerData'

export function PlayerMeta() {
  return (
    <div className="km-player-meta">
      <span>{playerData.state.qualityLabel}</span>
      <button type="button">Queue</button>
    </div>
  )
}
