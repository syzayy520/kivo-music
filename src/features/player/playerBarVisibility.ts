export type PlayerSessionState = 'none' | 'paused' | 'playing'

export type PlayerBarVisibility = 'hidden' | 'full'

export type ResolvePlayerBarVisibilityOptions = {
  isPremiumLocalPreviewPlayer?: boolean
}

export function resolvePlayerBarVisibility(
  sessionState: PlayerSessionState,
  options: ResolvePlayerBarVisibilityOptions = {},
): PlayerBarVisibility {
  if (sessionState !== 'none') {
    return 'full'
  }

  return options.isPremiumLocalPreviewPlayer ? 'full' : 'hidden'
}
