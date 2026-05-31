import type { CSSProperties } from 'react'
import { topBarTokens } from './topBarTokens'

type TopBarStyle = CSSProperties & Record<`--${string}`, string>

export const topBarStyle: TopBarStyle = {
  '--km-topbar-min-height': `${topBarTokens.minHeight}px`,
  '--km-topbar-title-min': `${topBarTokens.titleMinColumn}px`,
  '--km-topbar-search-min': `${topBarTokens.searchMinColumn}px`,
  '--km-topbar-search-max': `${topBarTokens.searchMaxColumn}px`,
  '--km-topbar-compact-title-min': `${topBarTokens.compactTitleMinColumn}px`,
  '--km-topbar-compact-search-min': `${topBarTokens.compactSearchMinColumn}px`,
  '--km-topbar-compact-search-max': `${topBarTokens.compactSearchMaxColumn}px`,
  '--km-topbar-gap': `${topBarTokens.gap}px`,
  '--km-topbar-padding-top': `${topBarTokens.paddingTop}px`,
  '--km-topbar-padding-bottom': `${topBarTokens.paddingBottom}px`,
  '--km-topbar-search-gap': `${topBarTokens.searchGap}px`,
  '--km-topbar-search-padding-block': `${topBarTokens.searchPaddingBlock}px`,
  '--km-topbar-search-padding-inline': `${topBarTokens.searchPaddingInline}px`,
  '--km-topbar-status-padding-block': `${topBarTokens.statusPaddingBlock}px`,
  '--km-topbar-status-padding-inline': `${topBarTokens.statusPaddingInline}px`,
  '--km-topbar-status-radius': `${topBarTokens.statusRadius}px`,
  '--km-topbar-status-dot': `${topBarTokens.statusDotSize}px`,
  '--km-topbar-status-dot-gap': `${topBarTokens.statusDotGap}px`,
}
