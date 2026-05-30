import './playerTrackInfo.css'
import { playerData } from '../playerData'

export function PlayerTrackInfo() {
  return (
    <div className="km-player-track">
      <div className="km-player-art" style={playerData.artworkStyle} aria-hidden="true" />
      <section className="km-player-copy">
        <strong>{playerData.track.title}</strong>
        <span>{playerData.artist?.name}</span>
      </section>
      <div className="km-player-track-actions" aria-label="Track actions">
        <button type="button" aria-label="Favorite track" className="km-player-action">
          ♥
        </button>
        <button type="button" aria-label="More track actions" className="km-player-action">
          ⋯
        </button>
      </div>
    </div>
  )
}
