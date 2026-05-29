import { tracks } from '../../data/mock/tracks'
import { playerState } from '../player/playerState'

export function LyricsPanel() {
  const track = tracks.find((item) => item.id === playerState.currentTrackId) ?? tracks[0]
  const lines = track.syncedLyrics ?? []

  return (
    <section className="km-lyrics" aria-label="Lyrics preview">
      <p>Synced Lyrics Preview</p>
      {lines.map((line) => (
        <div className={line.id === playerState.activeLyricsLineId ? 'active' : ''} key={line.id}>
          {line.text}
        </div>
      ))}
    </section>
  )
}
