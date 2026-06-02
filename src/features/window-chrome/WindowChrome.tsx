import { getCurrentWindow } from '@tauri-apps/api/window'
import type { MouseEvent } from 'react'
import { windowChromeStyle } from './windowChromeStyle'
import './windowChrome.css'
import './windowChromePremiumLocal.css'

type KivoWindow = ReturnType<typeof getCurrentWindow>

function getSafeCurrentWindow(): KivoWindow | null {
  try {
    return getCurrentWindow()
  } catch {
    return null
  }
}

function runWindowAction(action: (window: KivoWindow) => Promise<void>) {
  const currentWindow = getSafeCurrentWindow()

  if (!currentWindow) {
    return
  }

  void action(currentWindow).catch(() => undefined)
}

function startWindowDrag(event: MouseEvent<HTMLElement>) {
  if (event.button !== 0) {
    return
  }

  const target = event.target

  if (target instanceof HTMLElement && target.closest('button')) {
    return
  }

  runWindowAction((currentWindow) => currentWindow.startDragging())
}

function stopWindowControlPropagation(event: MouseEvent<HTMLButtonElement>) {
  event.stopPropagation()
}

export function WindowChrome() {
  return (
    <header
      className="km-window-chrome"
      data-tauri-drag-region
      style={windowChromeStyle}
      onDoubleClick={() => runWindowAction((currentWindow) => currentWindow.toggleMaximize())}
      onMouseDown={startWindowDrag}
    >
      <div className="km-window-traffic-lights" aria-hidden="true" data-tauri-drag-region>
        <span className="km-window-traffic-light km-window-traffic-light-close" />
        <span className="km-window-traffic-light km-window-traffic-light-minimize" />
        <span className="km-window-traffic-light km-window-traffic-light-zoom" />
      </div>
      <div className="km-window-drag" data-tauri-drag-region />
      <div className="km-window-controls">
        <button
          type="button"
          aria-label="Minimize"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => runWindowAction((currentWindow) => currentWindow.minimize())}
        >
          −
        </button>
        <button
          type="button"
          aria-label="Maximize"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => runWindowAction((currentWindow) => currentWindow.toggleMaximize())}
        >
          □
        </button>
        <button
          type="button"
          aria-label="Close"
          className="danger"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => runWindowAction((currentWindow) => currentWindow.close())}
        >
          ×
        </button>
      </div>
    </header>
  )
}
