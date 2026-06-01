import { t } from '../../shared/i18n'
import { lyricsData } from './lyricsData'
import './lyrics.css'

export function LyricsPanel() {
  return (
    <section className="km-lyrics" aria-label={t('lyrics.preview')}>
      <p>{t('lyrics.syncedLyrics')}</p>
      {lyricsData.lines.map((line) => (
        <div className={line.id === lyricsData.activeLineId ? 'active' : ''} key={line.id}>
          {line.text}
        </div>
      ))}
    </section>
  )
}
