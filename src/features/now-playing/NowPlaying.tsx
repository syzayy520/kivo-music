import { LyricsPanel } from '../lyrics/LyricsPanel'
import { QueuePreview } from '../queue/QueuePreview'
import './nowPlaying.css'

export function NowPlaying() {
  return (
    <section className="km-now-playing" id="now-playing">
      <div className="km-now-playing-head">
        <p>Now Playing</p>
        <h2>Lyrics and queue, kept light.</h2>
      </div>
      <LyricsPanel />
      <QueuePreview />
    </section>
  )
}
