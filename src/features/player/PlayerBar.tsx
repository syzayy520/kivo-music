import { artists } from '../../data/mock/artists'
import { tracks } from '../../data/mock/tracks'
import { playerState } from './playerState'

export function PlayerBar() {
  const track = tracks.find((item) => item.id === playerState.currentTrackId) ?? tracks[0]
  const artist = artists.find((item) => item.id === track.artistId)

  return (
    <footer className="km-player" aria-label="Player bar">
      <div className="km-player-track">
        <div />
        <section>
          <strong>{track.title}</strong>
          <span>{artist?.name}</span>
        </section>
      </div>
      <div className="km-player-core">
        <div className="km-player-buttons">
          <button type="button">Prev</button>
          <button type="button" className="play">Pause</button>
          <button type="button">Next</button>
        </div>
        <div className="km-progress"><span style={{ width: `${playerState.progress}%` }} /></div>
      </div>
      <div className="km-player-meta">
        <span>{playerState.qualityLabel}</span>
        <button type="button">Queue</button>
      </div>
    </footer>
  )
}
