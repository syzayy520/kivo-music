import { getCurrentWindow } from '@tauri-apps/api/window'
import type { MouseEvent } from 'react'
import { windowChromeStyle } from './windowChromeStyle'
import './windowChrome.css'

const appWindow = getCurrentWindow()

function startWindowDrag(event: MouseEvent<HTMLElement>) {
  if (event.button !== 0) {
    return
  }

  const target = event.target

  if (target instanceof HTMLElement && target.closest('button')) {
    return
  }

  void appWindow.startDragging()
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
      onDoubleClick={() => void appWindow.toggleMaximize()}
      onMouseDown={startWindowDrag}
    >
      <div className="km-window-drag" data-tauri-drag-region />
      <div className="km-window-controls">
        <button
          type="button"
          aria-label="Minimize"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => void appWindow.minimize()}
        >
          −
        </button>
        <button
          type="button"
          aria-label="Maximize"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => void appWindow.toggleMaximize()}
        >
          □
        </button>
        <button
          type="button"
          aria-label="Close"
          className="danger"
          onDoubleClick={stopWindowControlPropagation}
          onMouseDown={stopWindowControlPropagation}
          onClick={() => void appWindow.close()}
        >
          ×
        </button>
      </div>
    </header>
  )
}
