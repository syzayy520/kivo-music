export type PlayerControlVariant = 'subtle' | 'transport' | 'primary'

export type PlayerControlButton = {
  key: string
  label: string
  symbol: string
  variant: PlayerControlVariant
}

export const playerControlButtons: PlayerControlButton[] = [
  {
    key: 'shuffle',
    label: 'Shuffle',
    symbol: '↝',
    variant: 'subtle',
  },
  {
    key: 'previous',
    label: 'Previous track',
    symbol: '⏮',
    variant: 'transport',
  },
  {
    key: 'play-pause',
    label: 'Pause',
    symbol: 'Ⅱ',
    variant: 'primary',
  },
  {
    key: 'next',
    label: 'Next track',
    symbol: '⏭',
    variant: 'transport',
  },
  {
    key: 'repeat',
    label: 'Repeat',
    symbol: '↻',
    variant: 'subtle',
  },
]
