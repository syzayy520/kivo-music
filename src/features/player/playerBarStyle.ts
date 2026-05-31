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
  '--km-player-gap-min': `${playerTokens.gapMin}px`,
  '--km-player-gap-vw': `${playerTokens.gapViewport}vw`,
  '--km-player-gap-max': `${playerTokens.gapMax}px`,
  '--km-player-margin-inline-min': `${playerTokens.marginInlineMin}px`,
  '--km-player-margin-inline-vw': `${playerTokens.marginInlineViewport}vw`,
  '--km-player-margin-inline-max': `${playerTokens.marginInlineMax}px`,
  '--km-player-padding-block': `${playerTokens.paddingBlock}px`,
  '--km-player-padding-inline': `${playerTokens.paddingInline}px`,
  '--km-player-radius': `${playerTokens.radius}px`,
  '--km-player-compact-track-min': `${playerTokens.compactTrackColumn}px`,
  '--km-player-compact-track-max': `${playerTokens.compactTrackColumnMax}px`,
  '--km-player-compact-core-min': `${playerTokens.compactCoreColumn}px`,
  '--km-player-compact-gap': `${playerTokens.compactGap}px`,
}
