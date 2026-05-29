import { getCurrentWindow } from '@tauri-apps/api/window'
import './windowChrome.css'

const appWindow = getCurrentWindow()

export function WindowChrome() {
  return (
    <header className="km-window-chrome" data-tauri-drag-region>
      <div className="km-window-brand" data-tauri-drag-region>
        <span>K</span>
        <strong>Kivo Music</strong>
      </div>
      <div className="km-window-drag" data-tauri-drag-region />
      <div className="km-window-controls">
        <button type="button" aria-label="Minimize" onClick={() => void appWindow.minimize()}>
          −
        </button>
        <button type="button" aria-label="Maximize" onClick={() => void appWindow.toggleMaximize()}>
          □
        </button>
        <button type="button" aria-label="Close" className="danger" onClick={() => void appWindow.close()}>
          ×
        </button>
      </div>
    </header>
  )
}
