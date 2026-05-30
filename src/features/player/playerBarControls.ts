import type { PlayerIconName } from './playerIcons'

export type PlayerControlVariant = 'subtle' | 'transport' | 'primary'

export type PlayerControlButton = {
  key: string
  label: string
  icon: PlayerIconName
  variant: PlayerControlVariant
}

export const playerControlButtons: PlayerControlButton[] = [
  {
    key: 'shuffle',
    label: 'Shuffle',
    icon: 'shuffle',
    variant: 'subtle',
  },
  {
    key: 'previous',
    label: 'Previous track',
    icon: 'previous',
    variant: 'transport',
  },
  {
    key: 'play-pause',
    label: 'Pause',
    icon: 'pause',
    variant: 'primary',
  },
  {
    key: 'next',
    label: 'Next track',
    icon: 'next',
    variant: 'transport',
  },
  {
    key: 'repeat',
    label: 'Repeat',
    icon: 'repeat',
    variant: 'subtle',
  },
]
