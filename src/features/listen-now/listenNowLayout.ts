import { resolvePlayerBarVisibility } from '../player/playerBarVisibility'
import { playerData } from '../player/playerData'

export function resolveListenNowLayoutState() {
  return {
    playerBarVisibility: resolvePlayerBarVisibility(playerData.state.sessionState),
  } as const
}
