import './playerBar.css'
import { t } from '../../shared/i18n'
import { PlayerControls } from './components/PlayerControls'
import { PlayerMeta } from './components/PlayerMeta'
import { PlayerTrackInfo } from './components/PlayerTrackInfo'
import { resolvePlayerBarVisibility } from './playerBarVisibility'
import { playerData } from './playerData'

export function PlayerBar() {
  const visibility = resolvePlayerBarVisibility(playerData.state.sessionState)

  if (visibility !== 'full') {
    return <></>
  }

  return (
    <footer className="km-player" aria-label={t('player.bar.ariaLabel')}>
      <PlayerTrackInfo />
      <PlayerControls />
      <PlayerMeta />
    </footer>
  )
}
