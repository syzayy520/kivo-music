import { t } from '../../shared/i18n'
import { queueData } from './queueData'
import './queue.css'

export function QueuePreview() {
  return (
    <section className="km-queue">
      <div className="km-queue-head">
        <p>{t('queue.upNext')}</p>
        <button type="button">{t('player.queue')}</button>
      </div>
      {queueData.tracks.map((track) => (
        <article key={track.id}>
          <strong>{track.title}</strong>
          <span>{track.qualityLabel}</span>
        </article>
      ))}
    </section>
  )
}
