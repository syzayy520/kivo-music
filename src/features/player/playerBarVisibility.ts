export type PlayerSessionState = 'none' | 'paused' | 'playing'

export type PlayerBarVisibility = 'hidden' | 'full'

export function resolvePlayerBarVisibility(
  sessionState: PlayerSessionState,
): PlayerBarVisibility {
  return sessionState === 'none' ? 'hidden' : 'full'
}
