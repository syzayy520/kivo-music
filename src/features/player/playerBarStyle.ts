import type { CSSProperties } from 'react'
import { playerTokens } from '../../shared/design'

type PlayerBarStyle = CSSProperties & Record<`--${string}`, string>

export const playerBarStyle: PlayerBarStyle = {
  '--km-player-min-bar-height': `${playerTokens.minBarHeight}px`,
  '--km-player-floating-bottom': `${playerTokens.floatingBottom}px`,
  '--km-player-track-min': `${playerTokens.minTrackColumn}px`,
  '--km-player-track-max': `${playerTokens.maxTrackColumn}px`,
  '--km-player-core-min': `${playerTokens.minCoreColumn}px`,
  '--km-player-meta-min': `${playerTokens.minMetaColumn}px`,
  '--km-player-meta-max': `${playerTokens.maxMetaColumn}px`,
}
