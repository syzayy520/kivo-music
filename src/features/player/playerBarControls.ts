import type { TranslationKey } from '../../shared/i18n'
import type { PlayerIconName } from './playerIcons'

export type PlayerControlVariant = 'subtle' | 'transport' | 'primary'

export type PlayerControlButton = {
  key: string
  labelKey: TranslationKey
  icon: PlayerIconName
  variant: PlayerControlVariant
}

export const playerControlButtons: PlayerControlButton[] = [
  {
    key: 'shuffle',
    labelKey: 'player.controls.shuffle',
    icon: 'shuffle',
    variant: 'subtle',
  },
  {
    key: 'previous',
    labelKey: 'player.controls.previous',
    icon: 'previous',
    variant: 'transport',
  },
  {
    key: 'play-pause',
    labelKey: 'player.controls.pause',
    icon: 'pause',
    variant: 'primary',
  },
  {
    key: 'next',
    labelKey: 'player.controls.next',
    icon: 'next',
    variant: 'transport',
  },
  {
    key: 'repeat',
    labelKey: 'player.controls.repeat',
    icon: 'repeat',
    variant: 'subtle',
  },
]
