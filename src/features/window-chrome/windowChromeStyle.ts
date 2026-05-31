import type { CSSProperties } from 'react'
import { windowChromeTokens } from './windowChromeTokens'

type WindowChromeStyle = CSSProperties & Record<`--${string}`, string>

export const windowChromeStyle: WindowChromeStyle = {
  '--km-window-chrome-height': `${windowChromeTokens.height}px`,
  '--km-window-chrome-control-width': `${windowChromeTokens.controlWidth}px`,
  '--km-window-chrome-control-font-size': `${windowChromeTokens.controlFontSize}px`,
}
