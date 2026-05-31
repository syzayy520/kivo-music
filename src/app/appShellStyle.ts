import type { CSSProperties } from 'react'
import { shellTokens } from '../shared/design'

export const appShellStyle = {
  '--km-shell-min-width': `${shellTokens.appMinWidth}px`,
  '--km-shell-min-height': `${shellTokens.appMinHeight}px`,
  '--km-shell-top-row': `${shellTokens.topRowHeight}px`,
  '--km-shell-sidebar': `${shellTokens.sidebarWidth}px`,
  '--km-shell-sidebar-compact': `${shellTokens.sidebarCompactWidth}px`,
  '--km-shell-stage-bottom': `${shellTokens.stageBottomSpace}px`,
} as CSSProperties
