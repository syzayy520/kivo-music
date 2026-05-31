import type { CSSProperties } from 'react'
import { sidebarTokens } from './sidebarTokens'

type SidebarStyle = CSSProperties & Record<`--${string}`, string>

export const sidebarStyle: SidebarStyle = {
  '--km-sidebar-padding-block': `${sidebarTokens.paddingBlock}px`,
  '--km-sidebar-padding-inline': `${sidebarTokens.paddingInline}px`,
  '--km-sidebar-brand-gap': `${sidebarTokens.brandGap}px`,
  '--km-sidebar-brand-bottom': `${sidebarTokens.brandBottomSpace}px`,
  '--km-sidebar-brand-mark': `${sidebarTokens.brandMarkSize}px`,
  '--km-sidebar-brand-radius': `${sidebarTokens.brandMarkRadius}px`,
  '--km-sidebar-brand-name-size': `${sidebarTokens.brandNameFontSize}px`,
  '--km-sidebar-brand-tagline-size': `${sidebarTokens.brandTaglineFontSize}px`,
  '--km-sidebar-nav-gap': `${sidebarTokens.navGap}px`,
  '--km-sidebar-nav-label-bottom': `${sidebarTokens.navLabelBottomSpace}px`,
  '--km-sidebar-nav-label-size': `${sidebarTokens.navLabelFontSize}px`,
  '--km-sidebar-nav-item-margin-y': `${sidebarTokens.navItemVerticalMargin}px`,
  '--km-sidebar-nav-item-block': `${sidebarTokens.navItemBlockPadding}px`,
  '--km-sidebar-nav-item-inline': `${sidebarTokens.navItemInlinePadding}px`,
  '--km-sidebar-nav-item-radius': `${sidebarTokens.navItemRadius}px`,
  '--km-sidebar-nav-item-size': `${sidebarTokens.navItemFontSize}px`,
}
