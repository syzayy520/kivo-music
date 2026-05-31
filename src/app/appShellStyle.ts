import type { CSSProperties } from 'react'
import { shellTokens } from '../shared/design'

export const appShellStyle = {
  '--km-shell-min-width': `${shellTokens.appMinWidth}px`,
  '--km-shell-min-height': `${shellTokens.appMinHeight}px`,
  '--km-shell-top-row': `${shellTokens.topRowHeight}px`,
  '--km-shell-sidebar': `${shellTokens.sidebarWidth}px`,
  '--km-shell-sidebar-compact': `${shellTokens.sidebarCompactWidth}px`,
  '--km-shell-stage-bottom': `${shellTokens.stageBottomSpace}px`,
  '--km-shell-stage-scrollbar-width': `${shellTokens.stageScrollbarWidth}px`,
  '--km-shell-stage-scrollbar-thumb-border': `${shellTokens.stageScrollbarThumbBorder}px`,
  '--km-shell-stage-scrollbar-color': shellTokens.stageScrollbarColor,
  '--km-shell-stage-scrollbar-hover-color': shellTokens.stageScrollbarHoverColor,
  '--km-shell-dark-stage-scrollbar-color': shellTokens.darkStageScrollbarColor,
  '--km-shell-dark-stage-scrollbar-hover-color': shellTokens.darkStageScrollbarHoverColor,
} as CSSProperties
