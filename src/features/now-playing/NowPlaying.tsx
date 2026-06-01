import { t } from '../../shared/i18n'
import { LyricsPanel } from '../lyrics/LyricsPanel'
import { QueuePreview } from '../queue/QueuePreview'
import './nowPlaying.css'

export function NowPlaying() {
  return (
    <section className="km-now-playing" id="now-playing">
      <div className="km-now-playing-head">
        <p>{t('sidebar.nowPlaying')}</p>
        <h2>{t('nowPlaying.subtitle')}</h2>
      </div>
      <LyricsPanel />
      <QueuePreview />
    </section>
  )
}
