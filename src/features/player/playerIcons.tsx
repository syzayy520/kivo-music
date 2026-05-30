export type PlayerIconName = 'shuffle' | 'previous' | 'pause' | 'next' | 'repeat' | 'volume'

type PlayerIconProps = {
  name: PlayerIconName
}

export function PlayerIcon({ name }: PlayerIconProps) {
  switch (name) {
    case 'shuffle':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon">
          <path d="M4 7h3.4c2.1 0 3.3 1 4.4 2.6l.4.6" />
          <path d="M4 17h3.4c2.1 0 3.3-1 4.4-2.6l3.1-4.8C16 8 17.2 7 19.3 7H21" />
          <path d="M18.5 4.8 21 7l-2.5 2.2" />
          <path d="M18.5 14.8 21 17l-2.5 2.2" />
        </svg>
      )
    case 'previous':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon">
          <path d="M7 6v12" />
          <path d="m18 7-8 5 8 5V7Z" />
        </svg>
      )
    case 'pause':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon km-player-icon-pause">
          <path d="M9 7v10" />
          <path d="M15 7v10" />
        </svg>
      )
    case 'next':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon">
          <path d="M17 6v12" />
          <path d="m6 7 8 5-8 5V7Z" />
        </svg>
      )
    case 'repeat':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon">
          <path d="M6 8h9.5c2 0 3.5 1.5 3.5 3.5" />
          <path d="M8.5 5.8 6 8l2.5 2.2" />
          <path d="M18 16H8.5c-2 0-3.5-1.5-3.5-3.5" />
          <path d="M15.5 13.8 18 16l-2.5 2.2" />
        </svg>
      )
    case 'volume':
      return (
        <svg viewBox="0 0 24 24" aria-hidden="true" className="km-player-icon">
          <path d="M5 10v4h3l4 3V7l-4 3H5Z" />
          <path d="M16 9.5c.8.7 1.2 1.5 1.2 2.5s-.4 1.8-1.2 2.5" />
          <path d="M18.5 7.5c1.3 1.2 2 2.7 2 4.5s-.7 3.3-2 4.5" />
        </svg>
      )
  }
}
