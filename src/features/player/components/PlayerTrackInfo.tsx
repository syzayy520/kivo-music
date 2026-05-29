import { playerData } from '../playerData'

export function PlayerTrackInfo() {
  return (
    <div className="km-player-track">
      <div className="km-player-art" />
      <section>
        <strong>{playerData.track.title}</strong>
        <span>{playerData.artist?.name}</span>
      </section>
    </div>
  )
}
