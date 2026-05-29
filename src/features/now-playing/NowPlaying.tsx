import { LyricsPanel } from '../lyrics/LyricsPanel'
import { QueuePreview } from '../queue/QueuePreview'

export function NowPlaying() {
  return (
    <section className="km-now" id="now-playing">
      <div className="km-now-intro">
        <p>Now Playing</p>
        <h2>Lyrics, queue, and playback state stay independent.</h2>
      </div>
      <LyricsPanel />
      <QueuePreview />
    </section>
  )
}
