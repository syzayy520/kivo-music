import { playerData } from '../playerData'

export function PlayerTrackInfo() {
  return (
    <div className="km-player-track">
      <div />
      <section>
        <strong>{playerData.track.title}</strong>
        <span>{playerData.artist?.name}</span>
      </section>
    </div>
  )
}
