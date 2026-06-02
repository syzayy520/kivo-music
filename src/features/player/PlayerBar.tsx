import './playerBar.css'
import './playerBarPremiumLocal.css'
import { ACTIVE_HOME_EXPERIENCE_SHELL_MODE } from '../home/homeExperienceRegistry'
import { t } from '../../shared/i18n'
import { PlayerControls } from './components/PlayerControls'
import { PlayerMeta } from './components/PlayerMeta'
import { PlayerTrackInfo } from './components/PlayerTrackInfo'
import { playerBarStyle } from './playerBarStyle'
import { resolvePlayerBarVisibility } from './playerBarVisibility'
import { playerData } from './playerData'

const isPremiumLocalPreviewPlayer =
  ACTIVE_HOME_EXPERIENCE_SHELL_MODE === 'premiumLocal' && playerData.state.sessionState === 'none'

export function PlayerBar() {
  const visibility = resolvePlayerBarVisibility(playerData.state.sessionState, {
    isPremiumLocalPreviewPlayer,
  })

  if (visibility !== 'full') {
    return <></>
  }

  return (
    <footer
      className="km-player"
      data-player-preview={isPremiumLocalPreviewPlayer ? 'premiumLocal' : undefined}
      aria-label={t('player.bar.ariaLabel')}
      style={playerBarStyle}
    >
      <PlayerTrackInfo />
      <PlayerControls />
      <PlayerMeta />
    </footer>
  )
}
