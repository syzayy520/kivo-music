import { lyricsData } from './lyricsData'
import './lyrics.css'

export function LyricsPanel() {
  return (
    <section className="km-lyrics" aria-label="Lyrics preview">
      <p>Synced Lyrics</p>
      {lyricsData.lines.map((line) => (
        <div className={line.id === lyricsData.activeLineId ? 'active' : ''} key={line.id}>
          {line.text}
        </div>
      ))}
    </section>
  )
}
