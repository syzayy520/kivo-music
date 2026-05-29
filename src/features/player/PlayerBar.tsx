import './playerBar.css'
import { PlayerControls } from './components/PlayerControls'
import { PlayerMeta } from './components/PlayerMeta'
import { PlayerTrackInfo } from './components/PlayerTrackInfo'

export function PlayerBar() {
  return (
    <footer className="km-player" aria-label="Player bar">
      <PlayerTrackInfo />
      <PlayerControls />
      <PlayerMeta />
    </footer>
  )
}
