import { LyricsPanel } from '../lyrics/LyricsPanel'
import { QueuePreview } from '../queue/QueuePreview'
import './nowPlaying.css'

export function NowPlayingSection() {
  return (
    <section className="km-now-playing">
      <div>
        <p>Now Playing Foundation</p>
        <h2>Lyrics, queue, and playback state stay independent.</h2>
      </div>
      <LyricsPanel />
      <QueuePreview />
    </section>
  )
}
