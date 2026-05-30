import './homeStart.css'
import { listenNowCopy } from '../listenNowCopy'

export function StartListeningPanel() {
  return (
    <section className="km-home-start" aria-label="Start listening">
      <div>
        <p>{listenNowCopy.start.eyebrow}</p>
        <h2>{listenNowCopy.start.title}</h2>
        <span>{listenNowCopy.start.description}</span>
      </div>
      <div className="km-home-start-actions">
        <button type="button">{listenNowCopy.start.primaryAction}</button>
        <button type="button">{listenNowCopy.start.secondaryAction}</button>
      </div>
    </section>
  )
}
