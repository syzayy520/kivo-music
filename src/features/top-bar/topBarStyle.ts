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
  '--km-topbar-title-eyebrow-size': `${topBarTokens.titleEyebrowFontSize}px`,
  '--km-topbar-title-top': `${topBarTokens.titleTopSpace}px`,
  '--km-topbar-title-max-width': `${topBarTokens.titleMaxWidthCh}ch`,
  '--km-topbar-title-min-size': `${topBarTokens.titleMinFontSize}px`,
  '--km-topbar-title-max-size': `${topBarTokens.titleMaxFontSize}px`,
  '--km-topbar-compact-title-max-width': `${topBarTokens.compactTitleMaxWidthCh}ch`,
  '--km-topbar-compact-title-size': `${topBarTokens.compactTitleFontSize}px`,
  '--km-topbar-search-gap': `${topBarTokens.searchGap}px`,
  '--km-topbar-search-padding-block': `${topBarTokens.searchPaddingBlock}px`,
  '--km-topbar-search-padding-inline': `${topBarTokens.searchPaddingInline}px`,
  '--km-topbar-search-label-size': `${topBarTokens.searchLabelFontSize}px`,
  '--km-topbar-status-padding-block': `${topBarTokens.statusPaddingBlock}px`,
  '--km-topbar-status-padding-inline': `${topBarTokens.statusPaddingInline}px`,
  '--km-topbar-status-radius': `${topBarTokens.statusRadius}px`,
  '--km-topbar-status-label-size': `${topBarTokens.statusLabelFontSize}px`,
  '--km-topbar-status-value-top': `${topBarTokens.statusValueTopSpace}px`,
  '--km-topbar-status-value-size': `${topBarTokens.statusValueFontSize}px`,
  '--km-topbar-status-dot': `${topBarTokens.statusDotSize}px`,
  '--km-topbar-status-dot-gap': `${topBarTokens.statusDotGap}px`,
}
