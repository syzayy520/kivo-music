import type { ReactNode } from 'react'

export type PremiumLocalHomeIconName =
  | 'album'
  | 'library'
  | 'more'
  | 'next'
  | 'pause'
  | 'play'
  | 'previous'
  | 'queue'
  | 'search'
  | 'sparkle'
  | 'volume'

type PremiumLocalHomeIconProps = {
  name: PremiumLocalHomeIconName
}

export function PremiumLocalHomeIcon({ name }: PremiumLocalHomeIconProps) {
  switch (name) {
    case 'album':
      return <Icon><rect x="5" y="4" width="14" height="16" rx="3" /><path d="M9 8h6" /><path d="M9 12h4" /></Icon>
    case 'library':
      return <Icon><path d="M5 7.5 12 4l7 3.5-7 3.5-7-3.5Z" /><path d="m5 12 7 3.5L19 12" /><path d="m5 16.5 7 3.5 7-3.5" /></Icon>
    case 'more':
      return <Icon><path d="M7 12h.1" /><path d="M12 12h.1" /><path d="M17 12h.1" /></Icon>
    case 'next':
      return <Icon><path d="M17 6v12" /><path d="m6 7 8 5-8 5V7Z" /></Icon>
    case 'pause':
      return <Icon><path d="M9 7v10" /><path d="M15 7v10" /></Icon>
    case 'play':
      return <Icon><path d="m9 7 8 5-8 5V7Z" /></Icon>
    case 'previous':
      return <Icon><path d="M7 6v12" /><path d="m18 7-8 5 8 5V7Z" /></Icon>
    case 'queue':
      return <Icon><path d="M6 7h12" /><path d="M6 12h12" /><path d="M6 17h8" /></Icon>
    case 'search':
      return <Icon><circle cx="11" cy="11" r="5" /><path d="m16 16 3 3" /></Icon>
    case 'sparkle':
      return <Icon><path d="m12 4 1.4 4.3L18 10l-4.6 1.7L12 16l-1.4-4.3L6 10l4.6-1.7L12 4Z" /></Icon>
    case 'volume':
      return <Icon><path d="M5 10v4h3l4 3V7l-4 3H5Z" /><path d="M16 9.5c.8.7 1.2 1.5 1.2 2.5s-.4 1.8-1.2 2.5" /></Icon>
  }
}

function Icon({ children }: { children: ReactNode }) {
  return (
    <svg className="km-home-premium-icon" viewBox="0 0 24 24" aria-hidden="true">
      {children}
    </svg>
  )
}
